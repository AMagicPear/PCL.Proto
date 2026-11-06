//! 外部链接的安全性校验；打开浏览器是宿主应用的事。

/// 校验并归一化一个可以交给系统浏览器的 http(s) 地址。
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
