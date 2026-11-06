//! 用系统默认浏览器打开外部链接。

/// 校验后交给系统浏览器打开。
pub fn open(url: &str) -> Result<(), String> {
    let url = pcl_core::utils::url::validate(url)?;
    // ShellExecute avoids cmd.exe URL expansion; Unix launchers are waited on and reaped.
    #[cfg(windows)]
    let result = open::that_detached(url);
    #[cfg(not(windows))]
    let result = open::that(url);
    result.map_err(|error| error.to_string())
}

pub async fn open_async(url: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || open(&url))
        .await
        .map_err(|error| error.to_string())?
}
