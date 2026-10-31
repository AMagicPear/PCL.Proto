//! Open external links in the system browser instead of navigating the app WebView.

/// Validate and normalize an http(s) URL that is safe to hand to the system browser.
pub fn validate(url: &str) -> Result<String, String> {
    let parsed = reqwest::Url::parse(url).map_err(|error| error.to_string())?;
    if !matches!(parsed.scheme(), "http" | "https")
        || parsed.host_str().is_none()
        || !parsed.username().is_empty()
        || parsed.password().is_some()
    {
        return Err("Invalid external URL".into());
    }
    Ok(parsed.into())
}

/// Open a validated external URL with the platform's default browser.
pub fn open(url: &str) -> Result<(), String> {
    let url = validate(url)?;
    // ShellExecute avoids cmd.exe URL expansion; Unix launchers are waited on and reaped.
    #[cfg(windows)]
    let result = open::that_detached(url);
    #[cfg(not(windows))]
    let result = open::that(url);
    result.map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn open_external_url(url: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || open(&url))
        .await
        .map_err(|error| error.to_string())?
}

#[cfg(test)]
mod tests {
    use super::validate;

    #[test]
    fn accepts_http_links() {
        assert_eq!(
            validate("https://example.com/a?b=c").unwrap(),
            "https://example.com/a?b=c"
        );
        assert_eq!(
            validate("http://localhost:5173/").unwrap(),
            "http://localhost:5173/"
        );
    }

    #[test]
    fn rejects_unsafe_links() {
        for url in [
            "file:///etc/passwd",
            "javascript:alert(1)",
            "https://user:pass@example.com",
            "https://user@example.com",
            "https://example.com@evil.example/",
            "mailto:test@example.com",
            "data:text/html,hello",
            "//example.com/path",
            "/relative",
            "not a url",
        ] {
            assert!(validate(url).is_err(), "{url}");
        }
    }
}
