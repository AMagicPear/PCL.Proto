//! CE's vanilla installation stages, expressed as an async Rust service.
use super::{
    api_client::MinecraftApiClient,
    downloader::{DownloadConfig, Downloader, StageProgress, TaskItemReport, TaskStatus},
    minecraft::{DownloadInfo, RuleContext, VersionDetails, library_path, valid_sha1},
};
use crate::util::file::{safe_join, validate_file_name};
use anyhow::{Context, ensure};
use futures_util::{StreamExt, TryStreamExt};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::ipc::Channel;
use tokio::sync::{Mutex, mpsc};

/// Atomically reserves the destination; never removes an existing instance.
struct InstallDirectory {
    path: PathBuf,
    committed: bool,
}

impl InstallDirectory {
    fn create(repo: &Path, name: &str) -> anyhow::Result<Self> {
        validate_file_name(name)?;
        let versions = repo.join("versions");
        fs::create_dir_all(&versions)?;
        let path = versions.join(name);
        fs::create_dir(&path).with_context(|| {
            format!(
                "Cannot install: destination already exists or is unavailable: {}",
                path.display()
            )
        })?;
        Ok(Self {
            path,
            committed: false,
        })
    }
}

impl Drop for InstallDirectory {
    fn drop(&mut self) {
        if !self.committed
            && let Err(error) = fs::remove_dir_all(&self.path)
        {
            log::warn!(
                "Cannot remove incomplete installation {}: {error}",
                self.path.display()
            );
        }
    }
}

#[derive(Deserialize)]
struct AssetIndex {
    objects: BTreeMap<String, AssetObject>,
    #[serde(default, rename = "virtual")]
    virtual_assets: bool,
    #[serde(default)]
    map_to_resources: bool,
}

#[derive(Deserialize)]
struct AssetObject {
    hash: String,
    size: u64,
}

struct NativeArchive {
    path: PathBuf,
    exclude: Vec<String>,
}

struct InstallPlan {
    files: Vec<DownloadConfig>,
    natives: Vec<NativeArchive>,
    paths: HashMap<PathBuf, (String, u64)>,
    stages: BTreeMap<i32, Vec<u64>>,
}

impl InstallPlan {
    fn new() -> Self {
        Self {
            files: Vec::new(),
            natives: Vec::new(),
            paths: HashMap::new(),
            stages: [(1, vec![]), (2, vec![]), (3, vec![])].into(),
        }
    }

    fn add(&mut self, stage: i32, info: &DownloadInfo, path: PathBuf) -> anyhow::Result<()> {
        info.validate()?;
        if let Some((sha1, size)) = self.paths.get(&path) {
            ensure!(
                sha1.eq_ignore_ascii_case(&info.sha1) && *size == info.size,
                "Conflicting download metadata for {}",
                path.display()
            );
            return Ok(());
        }
        self.paths
            .insert(path.clone(), (info.sha1.clone(), info.size));
        let sizes = self
            .stages
            .get_mut(&stage)
            .context("Unknown install stage")?;
        self.files.push(DownloadConfig {
            file_index: sizes.len(),
            task_item_id: stage,
            out_path: path,
            info: info.clone(),
        });
        sizes.push(info.size);
        Ok(())
    }

    fn libraries(
        &mut self,
        details: &VersionDetails,
        repo: &Path,
        context: &RuleContext,
    ) -> anyhow::Result<()> {
        for library in &details.libraries {
            if !library.rule_allow(context)? {
                continue;
            }
            if let Some(info) = &library.downloads.artifact {
                self.add(2, info, library_path(repo, info)?)?;
            }
            if let Some(info) = library.native_download(context)? {
                let path = library_path(repo, info)?;
                self.add(2, info, path.clone())?;
                self.natives.push(NativeArchive {
                    path,
                    exclude: library
                        .extract
                        .as_ref()
                        .map(|rules| rules.exclude.clone())
                        .unwrap_or_default(),
                });
            }
        }
        if let Some(logging) = &details.logging {
            validate_file_name(&logging.client.file.id)?;
            self.add(
                2,
                &logging.client.file.download,
                repo.join("assets/log_configs")
                    .join(&logging.client.file.id),
            )?;
        }
        Ok(())
    }

    fn assets(
        &mut self,
        index: &AssetIndex,
        repo: &Path,
        resources_base: &str,
    ) -> anyhow::Result<()> {
        for (name, object) in &index.objects {
            // Both object hashes and legacy logical paths come from the network.
            safe_join(Path::new("assets"), name)?;
            ensure!(valid_sha1(&object.hash), "Invalid asset hash for {name}");
            let hash = object.hash.to_ascii_lowercase();
            let relative = format!("{}/{}", &hash[..2], hash);
            self.add(
                3,
                &DownloadInfo {
                    sha1: hash,
                    size: object.size,
                    url: format!("{resources_base}/{relative}"),
                    path: None,
                },
                repo.join("assets/objects").join(relative),
            )?;
        }
        Ok(())
    }
}

async fn download_plan(
    plan: &mut InstallPlan,
    downloader: &Downloader,
    task_id: i32,
    on_event: &Channel<TaskItemReport>,
) -> anyhow::Result<()> {
    let mut stages: BTreeMap<_, _> = std::mem::take(&mut plan.stages)
        .into_iter()
        .map(|(id, sizes)| (id, StageProgress::new(id, sizes)))
        .collect();
    for stage in stages.values_mut() {
        let _ = on_event.send(stage.report(task_id));
    }
    let (tx, mut rx) = mpsc::channel::<super::downloader::ProgressUpdate>(128);
    let download = async {
        let jobs = futures_util::stream::iter(std::mem::take(&mut plan.files).into_iter().map(
            |options| {
                let tx = tx.clone();
                async move {
                    let path = options.out_path.clone();
                    downloader
                        .start_download(options, tx)
                        .await
                        .with_context(|| format!("Cannot download {}", path.display()))
                }
            },
        ))
        // The shared quota controls requests, including live connection-limit changes.
        .buffer_unordered(256)
        .try_collect::<Vec<_>>()
        .await;
        drop(tx);
        jobs
    };
    let monitor = async {
        while let Some(update) = rx.recv().await {
            if let Some(stage) = stages.get_mut(&update.item_id) {
                stage.update(update.file_index, update.progress);
                if stage.should_report() {
                    let _ = on_event.send(stage.report(task_id));
                }
            }
        }
    };
    let (result, ()) = tokio::join!(download, monitor);
    result?;
    Ok(())
}

fn extract_natives(archives: &[NativeArchive], destination: &Path) -> anyhow::Result<()> {
    fs::create_dir_all(destination)?;
    for native in archives {
        let mut archive = zip::ZipArchive::new(fs::File::open(&native.path)?)?;
        for index in 0..archive.len() {
            let mut entry = archive.by_index(index)?;
            if entry.is_dir()
                || entry.name().starts_with("META-INF/")
                || native
                    .exclude
                    .iter()
                    .any(|exclude| entry.name().starts_with(exclude))
            {
                continue;
            }
            ensure!(
                entry
                    .unix_mode()
                    .is_none_or(|mode| mode & 0o170000 != 0o120000),
                "Native archive contains a symbolic link"
            );
            let path = safe_join(destination, entry.name())?;
            fs::create_dir_all(path.parent().context("Missing native parent")?)?;
            let mut file = fs::File::create(&path)?;
            std::io::copy(&mut entry, &mut file)?;
        }
    }
    Ok(())
}

fn materialize_assets(
    index: &AssetIndex,
    repo: &Path,
    instance: &Path,
    index_id: &str,
) -> anyhow::Result<()> {
    let mut roots = Vec::new();
    if index.virtual_assets {
        roots.push(repo.join("assets/virtual").join(index_id));
    }
    if index.map_to_resources {
        roots.push(instance.join("resources"));
    }
    for root in roots {
        for (name, object) in &index.objects {
            let hash = object.hash.to_ascii_lowercase();
            let source = repo.join("assets/objects").join(&hash[..2]).join(&hash);
            let target = safe_join(&root, name)?;
            fs::create_dir_all(target.parent().context("Missing asset parent")?)?;
            fs::copy(source, target)?;
        }
    }
    Ok(())
}

async fn install_version(
    client: &MinecraftApiClient,
    repo: &Path,
    version_id: &str,
    name: &str,
    task_id: i32,
    on_event: &Channel<TaskItemReport>,
) -> anyhow::Result<()> {
    validate_file_name(version_id)?;
    ensure!(
        repo.is_absolute() && repo.is_dir(),
        "Minecraft folder must be an existing absolute directory"
    );
    let repo = repo.canonicalize()?;
    let mut directory = InstallDirectory::create(&repo, name)?;
    log::info!(
        "Installing Minecraft {version_id} as {name} in {}",
        repo.display()
    );
    let _ = on_event.send(TaskItemReport {
        task_id,
        item_id: 0,
        files_remaining: 1,
        progress: 0.0,
        status: TaskStatus::Running,
        speed: None,
    });
    let mut metadata = client.get_version_metadata(version_id).await?;
    let details: VersionDetails =
        serde_json::from_value(metadata.clone()).context("Invalid vanilla version metadata")?;
    validate_file_name(&details.asset_index.id)?;
    details.asset_index.download.validate()?;
    let bases = client.api_bases_async().await;
    let downloader = Downloader::with_bases(bases.clone());
    let index_path = repo
        .join("assets/indexes")
        .join(format!("{}.json", details.asset_index.id));
    downloader
        .download(&details.asset_index.download, &index_path)
        .await?;
    let index: AssetIndex = serde_json::from_slice(&tokio::fs::read(&index_path).await?)
        .context("Invalid asset index")?;
    let mut plan = InstallPlan::new();
    plan.add(
        1,
        &details.downloads.client,
        directory.path.join(format!("{name}.jar")),
    )?;
    plan.libraries(&details, &repo, &RuleContext::current())?;
    plan.assets(&index, &repo, "https://resources.download.minecraft.net")?;
    let _ = on_event.send(TaskItemReport {
        task_id,
        item_id: 0,
        files_remaining: 0,
        progress: 1.0,
        status: TaskStatus::Completed,
        speed: None,
    });
    download_plan(&mut plan, &downloader, task_id, on_event).await?;
    let instance_path = directory.path.clone();
    let index_id = details.asset_index.id;
    tokio::task::spawn_blocking(move || {
        extract_natives(&plan.natives, &instance_path.join("natives"))?;
        materialize_assets(&index, &repo, &instance_path, &index_id)
    })
    .await??;
    // CE preserves the source version separately when the instance has a custom name.
    metadata["id"] = name.into();
    metadata["clientVersion"] = version_id.into();
    let temporary_json = directory.path.join(".pcl-version.json.part");
    tokio::fs::write(&temporary_json, serde_json::to_vec_pretty(&metadata)?).await?;
    // The instance becomes discoverable only after every install stage succeeds.
    tokio::fs::rename(temporary_json, directory.path.join(format!("{name}.json"))).await?;
    directory.committed = true;
    log::info!(
        "Installed Minecraft {version_id}: {}",
        directory.path.display()
    );
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn download_minecraft_version(
    state: tauri::State<'_, Arc<Mutex<crate::setup::AppState>>>,
    client: tauri::State<'_, &MinecraftApiClient>,
    on_event: Channel<TaskItemReport>,
    version_id: String,
    task_id: i32,
    instance_name: String,
    download_settings: Option<super::downloader::resources::DownloadSettings>,
) -> Result<(), String> {
    if let Some(settings) = download_settings {
        super::downloader::resources::DOWNLOAD_RESOURCES.configure(settings)?;
    }
    let repo = state.lock().await.active_repo_path.clone();
    install_version(
        &client,
        &repo,
        &version_id,
        &instance_name,
        task_id,
        &on_event,
    )
    .await
    .map_err(|error| format!("{error:#}"))
}

#[cfg(test)]
mod tests;
