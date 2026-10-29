use super::*;
use std::collections::{HashMap, HashSet, VecDeque};

const KEYWORDS: &[&str] = &[
    "java",
    "jdk",
    "jre",
    "dragonwell",
    "azul",
    "zulu",
    "oracle",
    "open",
    "amazon",
    "corretto",
    "eclipse",
    "temurin",
    "hotspot",
    "semeru",
    "kona",
    "bellsoft",
    "environment",
    "env",
    "runtime",
    "x86_64",
    "amd64",
    "arm64",
    "x64",
    "pcl",
    "hmcl",
    "baka",
    "minecraft",
    "contents",
    "home",
    "bin",
    "current",
    "candidate",
    "versions",
    "linux",
    "windows",
    "mac-os",
    "libexec",
    "jbr",
];
const EXCLUDED: &[&str] = &[
    "javapath",
    "java8path",
    "common files",
    "netease",
    "node_modules",
    ".git",
];

pub(super) async fn scan(
    known: Vec<JavaRuntime>,
    roots: Vec<PathBuf>,
) -> Result<Vec<JavaRuntime>, String> {
    tokio::task::spawn_blocking(move || scan_sync(known, roots))
        .await
        .map_err(|e| format!("Java scan failed: {e}"))
}
fn scan_sync(known: Vec<JavaRuntime>, extra_roots: Vec<PathBuf>) -> Vec<JavaRuntime> {
    let mut paths = HashSet::new();
    let mut roots = extra_roots;
    for key in ["JAVA_HOME", "JDK_HOME", "JRE_HOME"] {
        if let Some(home) = std::env::var_os(key) {
            paths.insert(PathBuf::from(home).join("bin").join(java_binary()));
        }
    }
    if let Some(value) = std::env::var_os("PATH") {
        paths.extend(std::env::split_paths(&value).map(|p| p.join(java_binary())));
    }
    if let Some(home) = std::env::home_dir() {
        roots.extend([
            home.join(".sdkman/candidates/java"),
            home.join(".asdf/installs/java"),
            home.join(".jabba/jdk"),
            home.join(".jdks"),
            home.join(".local/share/mise/installs/java"),
            home.join(".minecraft/runtime"),
        ]);
        #[cfg(target_os = "macos")]
        roots.extend([
            home.join("Library/Java/JavaVirtualMachines"),
            home.join("Library/Application Support/minecraft/runtime"),
            home.join("Applications"),
        ]);
    }
    if let Ok(directory) = launcher_directory() {
        roots.extend([
            directory.join("PCL"),
            directory.join("runtime"),
            directory.join("jre"),
            directory.join(".minecraft/runtime"),
        ]);
    }
    #[cfg(target_os = "macos")]
    roots.extend(
        [
            "/Applications",
            "/Library/Java/JavaVirtualMachines",
            "/System/Library/Java/JavaVirtualMachines",
            "/opt/homebrew/opt",
            "/usr/local/opt",
        ]
        .map(PathBuf::from),
    );
    #[cfg(target_os = "linux")]
    roots.extend(
        [
            "/usr/lib/jvm",
            "/usr/lib64/jvm",
            "/usr/java",
            "/opt/java",
            "/opt/jdk",
            "/opt",
        ]
        .map(PathBuf::from),
    );
    #[cfg(windows)]
    {
        for key in [
            "ProgramFiles",
            "ProgramFiles(x86)",
            "ProgramW6432",
            "LOCALAPPDATA",
            "APPDATA",
        ] {
            if let Some(root) = std::env::var_os(key) {
                roots.push(PathBuf::from(root));
            }
        }
        if let Some(local) = std::env::var_os("LOCALAPPDATA") {
            roots.push(
                PathBuf::from(local)
                    .join("Packages")
                    .join("Microsoft.4297127D64EC6_8wekyb3d8bbwe")
                    .join("LocalCache/Local/runtime"),
            );
        }
        for key in [
            r"HKLM\SOFTWARE\JavaSoft",
            r"HKLM\SOFTWARE\WOW6432Node\JavaSoft",
            r"HKLM\SOFTWARE\Eclipse Adoptium",
            r"HKLM\SOFTWARE\Azul Systems",
            r"HKLM\SOFTWARE\Microsoft\JDK",
            r"HKLM\SOFTWARE\BellSoft\Liberica",
        ] {
            // reg.exe follows the console code page; request UTF-8 for non-ASCII install paths.
            let mut command = Command::new("cmd.exe");
            command.args([
                "/D",
                "/C",
                &format!("chcp 65001>nul & reg.exe query \"{key}\" /s"),
            ]);
            if let Ok(output) = command_output(command) {
                let text = String::from_utf8_lossy(&output.stdout);
                for line in text.lines() {
                    if let Some((_, value)) = line.split_once("REG_SZ") {
                        roots.push(PathBuf::from(value.trim()));
                    }
                }
            }
        }
    }
    paths.extend(discover_paths(roots));
    probe_paths(&known, paths)
}

pub(super) fn probe_paths(known: &[JavaRuntime], paths: HashSet<PathBuf>) -> Vec<JavaRuntime> {
    let mut results: HashMap<PathBuf, JavaRuntime> = HashMap::new();
    let mut probed = HashSet::new();
    // Probe saved paths first so discovery through another alias cannot re-enable a disabled runtime.
    for path in known.iter().map(|j| j.java_exe.clone()).chain(paths) {
        if !probed.insert(path.clone()) {
            continue;
        }
        if !path.is_file()
            || path.components().any(|p| {
                EXCLUDED.contains(&p.as_os_str().to_string_lossy().to_lowercase().as_str())
            })
        {
            continue;
        }
        match JavaRuntime::probe(&path) {
            Ok(mut runtime) => {
                if let Some(old) = known
                    .iter()
                    .find(|j| j.java_exe == runtime.java_exe || j.java_exe == path)
                {
                    runtime.is_enabled = old.is_enabled;
                    runtime.is_user_imported = old.is_user_imported;
                }
                results.entry(runtime.java_exe.clone()).or_insert(runtime);
            }
            Err(e) => log::debug!("Skipping Java {}: {e}", path.display()),
        }
    }
    let mut list = results.into_values().collect::<Vec<_>>();
    sort_list(&mut list);
    list
}

pub(super) fn discover_paths(roots: Vec<PathBuf>) -> HashSet<PathBuf> {
    let mut found = HashSet::new();
    let mut visited = HashSet::new();
    let mut queue: VecDeque<_> = roots.into_iter().map(|p| (p, 0)).collect();
    while let Some((path, depth)) = queue.pop_front() {
        let Ok(path) = path.canonicalize() else {
            continue;
        };
        if !visited.insert(path.clone()) || depth > 6 {
            continue;
        }
        for suffix in [
            PathBuf::from(java_binary()),
            PathBuf::from("bin").join(java_binary()),
            PathBuf::from("Contents/Home/bin").join(java_binary()),
        ] {
            let exe = path.join(suffix);
            if exe.is_file() {
                found.insert(exe);
            }
        }
        let Ok(entries) = std::fs::read_dir(&path) else {
            continue;
        };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_lowercase();
            if EXCLUDED.contains(&name.as_str()) {
                continue;
            }
            let version_like = name.chars().any(|c| c.is_ascii_digit())
                && name
                    .chars()
                    .all(|c| c.is_ascii_digit() || "._-+".contains(c));
            if entry.path().is_dir()
                && (depth == 0 || version_like || KEYWORDS.iter().any(|k| name.contains(k)))
            {
                queue.push_back((entry.path(), depth + 1));
            }
        }
    }
    found
}
