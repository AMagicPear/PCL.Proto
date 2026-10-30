use crate::core::{downloader::Downloader, minecraft::DownloadInfo};
use anyhow::{Context, ensure};
use std::path::{Path, PathBuf};

fn urls(loader: &str, game: &str, version: &str) -> anyhow::Result<(String, String)> {
    for value in [game, version] {
        ensure!(
            !value.is_empty()
                && value.len() <= 100
                && value
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"._+-".contains(&b))
                && value != "."
                && value != "..",
            "Invalid version"
        );
    }
    match loader {
        "forge" => {
            let id = format!("{game}-{version}");
            Ok((
                format!(
                    "https://maven.minecraftforge.net/net/minecraftforge/forge/{id}/forge-{id}-installer.jar"
                ),
                format!(
                    "https://files.minecraftforge.net/maven/net/minecraftforge/forge/{id}/forge-{id}-changelog.txt"
                ),
            ))
        }
        "fabric" | "legacyfabric" => Ok((
            format!(
                "https://maven.fabricmc.net/net/fabricmc/fabric-loader/{version}/fabric-loader-{version}.jar"
            ),
            "https://fabricmc.net/blog".into(),
        )),
        _ => anyhow::bail!("Unsupported loader"),
    }
}

struct TempFile(PathBuf);
impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[tauri::command]
pub async fn save_loader_file(
    loader: String,
    game: String,
    version: String,
    destination: PathBuf,
) -> Result<(), String> {
    async {
        let (url, _) = urls(&loader, &game, &version)?;
        let parent = destination
            .parent()
            .filter(|p| p.is_dir())
            .context("Invalid destination")?;
        ensure!(destination.file_name().is_some(), "Invalid destination");
        let client = reqwest::Client::builder()
            .user_agent(crate::setup::constants::USER_AGENT)
            .build()?;
        let sha1 = client
            .get(format!("{url}.sha1"))
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?
            .trim()
            .to_owned();
        let size = client
            .head(&url)
            .send()
            .await?
            .error_for_status()?
            .content_length()
            .context("Missing file size")?;
        ensure!(size > 0 && size <= 64 * 1024 * 1024, "Invalid file size");
        let temp = TempFile(parent.join(format!(".pcl-loader-{}.tmp", uuid::Uuid::new_v4())));
        Downloader::default()
            .download(
                &DownloadInfo {
                    sha1,
                    size,
                    url,
                    path: None,
                },
                Path::new(&temp.0),
            )
            .await?;
        std::fs::rename(&temp.0, &destination).context("Cannot save downloaded file")?;
        Ok::<(), anyhow::Error>(())
    }
    .await
    .map_err(|error| format!("{error:#}"))
}

#[tauri::command]
pub async fn open_loader_changelog(
    loader: String,
    game: String,
    version: String,
) -> Result<(), String> {
    let (_, url) = urls(&loader, &game, &version).map_err(|e| e.to_string())?;
    tauri::async_runtime::spawn_blocking(move || {
        let program = if cfg!(windows) {
            "explorer"
        } else if cfg!(target_os = "macos") {
            "open"
        } else {
            "xdg-open"
        };
        let status = std::process::Command::new(program)
            .arg(url)
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() {
            Ok(())
        } else {
            Err(format!("Cannot open changelog: {status}"))
        }
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn loader_urls_are_bounded_to_known_sources() {
        assert!(
            urls("forge", "1.20.1", "47.2.0")
                .unwrap()
                .0
                .ends_with("forge-1.20.1-47.2.0-installer.jar")
        );
        assert!(
            urls("fabric", "1.20.1", "0.16.10")
                .unwrap()
                .0
                .ends_with("fabric-loader-0.16.10.jar")
        );
        assert!(urls("forge", "../bad", "47").is_err());
        assert!(urls("unknown", "1.20.1", "47").is_err());
    }
}
