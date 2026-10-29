//! CE Java management, with executable probing and discovery for each desktop OS.
use super::platform::{Architecture, Compability};
use anyhow::{Context, ensure};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant},
};

mod discovery;
pub mod install;
pub mod requirements;
pub mod selection;
#[cfg(test)]
mod tests;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JavaRuntime {
    pub directory_path: PathBuf,
    pub is_user_imported: bool,
    pub version: String,
    pub slug_version: i32,
    pub is_64_bit: bool,
    pub architecture: Architecture,
    pub compability: Compability,
    pub is_jdk: bool,
    pub java_exe: PathBuf,
    pub implementor: Option<String>,
    #[serde(default = "enabled_by_default")]
    pub is_enabled: bool,
}
fn enabled_by_default() -> bool {
    true
}

impl PartialEq for JavaRuntime {
    fn eq(&self, other: &Self) -> bool {
        self.java_exe == other.java_exe
    }
}

/// Normalized Java version: 1.8.0_321 and 8.0.321 compare identically.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct JavaVersion(pub u32, pub u32, pub u32, pub u32);
impl JavaVersion {
    pub const fn major(major: u32) -> Self {
        Self(major, 0, 0, 0)
    }
    pub const fn upper(major: u32) -> Self {
        Self(major, u32::MAX, u32::MAX, u32::MAX)
    }
    pub fn parse(value: &str) -> Option<Self> {
        let numeric = value.trim().split(['-', '+']).next()?;
        let parts: Vec<u32> = numeric
            .split(['.', '_'])
            .map(str::parse)
            .collect::<Result<_, _>>()
            .ok()?;
        if parts.is_empty() || parts.len() > 4 {
            return None;
        }
        let n = |i| parts.get(i).copied().unwrap_or(0);
        if n(0) == 1 {
            (n(1) > 0).then_some(Self(n(1), n(2), n(3), 0))
        } else {
            Some(Self(n(0), n(1), n(2), n(3)))
        }
    }
}
impl std::fmt::Display for JavaVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.1 == u32::MAX {
            write!(f, "{}.*", self.0)
        } else {
            write!(f, "{}.{}.{}", self.0, self.1, self.2)
        }
    }
}

pub fn java_binary() -> &'static str {
    if cfg!(windows) { "java.exe" } else { "java" }
}
pub fn launcher_directory() -> anyhow::Result<PathBuf> {
    Ok(std::env::current_exe()?
        .parent()
        .context("Missing launcher directory")?
        .to_owned())
}

pub(super) fn command_output(mut command: Command) -> anyhow::Result<std::process::Output> {
    use std::io::Read;
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    // Drain both pipes while the process runs; registry scans can exceed a pipe's capacity.
    let (send, receive) = std::sync::mpsc::channel();
    for (is_stdout, mut pipe) in [
        (
            true,
            Box::new(child.stdout.take().unwrap()) as Box<dyn Read + Send>,
        ),
        (
            false,
            Box::new(child.stderr.take().unwrap()) as Box<dyn Read + Send>,
        ),
    ] {
        let send = send.clone();
        std::thread::spawn(move || {
            let mut bytes = Vec::new();
            let result = pipe
                .by_ref()
                .take(1024 * 1024 + 1)
                .read_to_end(&mut bytes)
                .map(|_| bytes);
            let _ = send.send((is_stdout, result));
        });
    }
    let (mut stdout, mut stderr) = (None, None);
    let start = Instant::now();
    loop {
        while let Ok((is_stdout, result)) = receive.try_recv() {
            if is_stdout {
                stdout = Some(result);
            } else {
                stderr = Some(result);
            }
        }
        if let Some(status) = child.try_wait()?
            && stdout.is_some()
            && stderr.is_some()
        {
            let (stdout, stderr) = (stdout.unwrap()?, stderr.unwrap()?);
            ensure!(
                stdout.len() <= 1024 * 1024 && stderr.len() <= 1024 * 1024,
                "Java probe output is too large"
            );
            return Ok(std::process::Output {
                status,
                stdout,
                stderr,
            });
        }
        if start.elapsed() > Duration::from_secs(5) {
            let _ = child.kill();
            let _ = child.wait();
            anyhow::bail!("Java probe timed out");
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

impl JavaRuntime {
    pub fn probe(java_path: &Path) -> anyhow::Result<Self> {
        ensure!(
            java_path.is_file(),
            "Java executable does not exist: {}",
            java_path.display()
        );
        let mut command = Command::new(java_path);
        command.args(["-XshowSettings:properties", "-version"]);
        let output = command_output(command)?;
        let text = |output: &std::process::Output| {
            format!(
                "{}\n{}",
                String::from_utf8_lossy(&output.stderr),
                String::from_utf8_lossy(&output.stdout)
            )
        };
        if output.status.success()
            && let Ok(runtime) = Self::from_probe(java_path, &text(&output))
        {
            return Ok(runtime);
        }
        // Java 6 and some older vendor builds do not implement -XshowSettings.
        let mut command = Command::new(java_path);
        command.arg("-version");
        let output = command_output(command)?;
        ensure!(
            output.status.success(),
            "Java executable could not start: {}",
            java_path.display()
        );
        Self::from_probe(java_path, &text(&output))
    }

    fn from_probe(java_path: &Path, text: &str) -> anyhow::Result<Self> {
        let property = |name: &str| {
            text.lines().find_map(|line| {
                let (key, value) = line.trim().split_once('=')?;
                (key.trim() == name).then(|| value.trim().to_owned())
            })
        };
        let version = property("java.version")
            .or_else(|| {
                let re = regex::Regex::new(r#"(?:openjdk|java) version\s+"([^"]+)"#).unwrap();
                re.captures(text).map(|v| v[1].to_owned())
            })
            .context("Cannot identify Java version")?;
        let parsed = JavaVersion::parse(&version).context("Invalid Java version")?;
        ensure!(parsed.0 > 0, "Invalid Java major version");
        let home = property("java.home").map(PathBuf::from).map(|home| {
            // Java 8 JDKs report their nested jre as java.home; retain the JDK executable and its javac.
            if home.file_name().is_some_and(|name| name == "jre")
                && let Some(parent) = home.parent()
                && parent
                    .join("bin")
                    .join(if cfg!(windows) { "javac.exe" } else { "javac" })
                    .is_file()
            {
                return parent.to_owned();
            }
            home
        });
        let actual = home
            .as_ref()
            .map(|p| p.join("bin").join(java_binary()))
            .filter(|p| p.is_file())
            .unwrap_or_else(|| java_path.to_owned());
        let java_exe = actual
            .canonicalize()
            .with_context(|| format!("Invalid Java path: {}", actual.display()))?;
        let directory_path = java_exe
            .parent()
            .context("Missing Java directory")?
            .to_owned();
        let architecture = match property("os.arch").as_deref() {
            Some("amd64" | "x86_64" | "x64") => Architecture::X64,
            Some("aarch64" | "arm64") => Architecture::Arm64,
            Some("x86" | "i386" | "i486" | "i586" | "i686") => Architecture::X86,
            _ => Architecture::read_header(&java_exe).unwrap_or(Architecture::Unknown),
        };
        let is_64_bit = property("sun.arch.data.model")
            .map(|v| v == "64")
            .unwrap_or(matches!(
                architecture,
                Architecture::Arm64 | Architecture::X64 | Architecture::FatFile
            ));
        let compability = super::platform::assert_compability(&architecture);
        let is_jdk = directory_path
            .join(if cfg!(windows) { "javac.exe" } else { "javac" })
            .is_file();
        let is_enabled = !matches!(compability, Compability::No | Compability::Unknown)
            && is_64_bit == matches!(super::platform::host_architecture(), "x86_64" | "aarch64")
            && (is_jdk || parsed.0 <= 8);
        Ok(Self {
            directory_path,
            is_user_imported: false,
            version,
            slug_version: parsed.0 as i32,
            is_64_bit,
            architecture,
            compability,
            is_jdk,
            java_exe,
            implementor: property("java.vendor"),
            is_enabled,
        })
    }
    pub fn parsed_version(&self) -> JavaVersion {
        JavaVersion::parse(&self.version).unwrap_or(JavaVersion::major(0))
    }
    pub fn brand_rank(&self) -> usize {
        let vendor = self.implementor.as_deref().unwrap_or("").to_lowercase();
        [
            "eclipse",
            "bellsoft",
            "azul",
            "amazon",
            "microsoft",
            "ibm",
            "oracle",
            "alibaba",
            "tencent",
            "openjdk",
            "graalvm",
            "jetbrains",
        ]
        .iter()
        .position(|brand| vendor.contains(brand))
        .unwrap_or(12)
    }
}
impl TryFrom<&str> for JavaRuntime {
    type Error = anyhow::Error;
    fn try_from(path: &str) -> Result<Self, Self::Error> {
        Self::probe(Path::new(path))
    }
}

pub fn sort_list(list: &mut [JavaRuntime]) {
    list.sort_by(|a, b| {
        b.parsed_version()
            .cmp(&a.parsed_version())
            .then(a.brand_rank().cmp(&b.brand_rank()))
            .then(a.java_exe.cmp(&b.java_exe))
    });
}

pub(super) fn merge_runtimes(
    mut scanned: Vec<JavaRuntime>,
    before: &[JavaRuntime],
    current: &[JavaRuntime],
) -> Vec<JavaRuntime> {
    for runtime in &mut scanned {
        if let Some(saved) = current.iter().find(|j| j.java_exe == runtime.java_exe) {
            runtime.is_enabled = saved.is_enabled;
            runtime.is_user_imported |= saved.is_user_imported;
        }
    }
    // Imports made during a scan must survive, while unavailable entries in the scan snapshot disappear.
    for runtime in current {
        if !before.iter().any(|j| j.java_exe == runtime.java_exe)
            && !scanned.iter().any(|j| j.java_exe == runtime.java_exe)
        {
            scanned.push(runtime.clone());
        }
    }
    sort_list(&mut scanned);
    scanned
}

static SCAN_LOCK: tokio::sync::Mutex<Option<Instant>> = tokio::sync::Mutex::const_new(None);
pub async fn refresh(force: bool) -> Result<Vec<JavaRuntime>, String> {
    let mut scan = SCAN_LOCK.lock().await;
    let config = crate::setup::ConfigManager::instance();
    if !force && scan.is_some_and(|last| last.elapsed() < Duration::from_secs(13)) {
        return Ok(config.app_state.lock().await.java_runtimes.clone());
    }
    let (known, mut roots, selected) = {
        let state = config.app_state.lock().await;
        let known = state.java_runtimes.clone();
        let roots = state
            .repositories
            .iter()
            .map(|r| r.path.join("runtime"))
            .collect::<Vec<_>>();
        (known, roots, state.pcl_setup_info.selected_java.clone())
    };
    roots.push(config.java_runtime_dir());
    let scanned = discovery::scan(known.clone(), roots).await?;
    let selected_probe = selected.clone();
    let actual_selected = tokio::task::spawn_blocking(move || {
        selected_probe
            .and_then(|p| JavaRuntime::probe(&p).ok())
            .map(|j| j.java_exe)
    })
    .await
    .map_err(|e| e.to_string())?;
    {
        let mut state = config.app_state.lock().await;
        state.java_runtimes = merge_runtimes(scanned, &known, &state.java_runtimes);
        if state.pcl_setup_info.selected_java == selected && actual_selected.is_some() {
            state.pcl_setup_info.selected_java = actual_selected;
        }
    }
    tauri::async_runtime::spawn_blocking(move || config.save().map_err(|e| format!("{e:?}")))
        .await
        .map_err(|e| e.to_string())??;
    *scan = Some(Instant::now());
    Ok(config.app_state.lock().await.java_runtimes.clone())
}
