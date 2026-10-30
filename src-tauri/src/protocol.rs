use std::{
    borrow::Cow,
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{LazyLock, Mutex},
};

use tauri::http::{Request, Response, StatusCode};

/// Files that the current listings exposed to the webview. The custom protocol only serves
/// tokens registered here, so a page cannot read arbitrary paths even if it runs injected script.
static SERVED: LazyLock<Mutex<HashMap<String, PathBuf>>> = LazyLock::new(Default::default);

const MAX_TOKENS: usize = 20_000;
const MAX_FILE_SIZE: u64 = 64 * 1024 * 1024;

pub fn register(path: &Path) -> String {
    let token = uuid::Uuid::new_v4().to_string();
    if let Ok(mut served) = SERVED.lock() {
        if served.len() >= MAX_TOKENS {
            served.clear();
        }
        served.insert(token.clone(), path.to_path_buf());
    }
    token
}

fn mime(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase()
        .as_str()
    {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "gif" => "image/gif",
        _ => "application/octet-stream",
    }
}

pub fn serve(request: Request<Vec<u8>>) -> Response<Cow<'static, [u8]>> {
    let token = request.uri().path().trim_start_matches('/').to_owned();
    let path = SERVED
        .lock()
        .ok()
        .and_then(|served| served.get(&token).cloned());
    let Some(path) = path else {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Cow::Owned(Vec::new()))
            .expect("static response");
    };
    let Ok(metadata) = std::fs::metadata(&path) else {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Cow::Owned(Vec::new()))
            .expect("static response");
    };
    if !metadata.is_file() || metadata.len() > MAX_FILE_SIZE {
        return Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Cow::Owned(Vec::new()))
            .expect("static response");
    }
    match std::fs::read(&path) {
        Ok(bytes) => Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", mime(&path))
            .header("Cache-Control", "no-store")
            .body(Cow::Owned(bytes))
            .expect("static response"),
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Cow::Owned(Vec::new()))
            .expect("static response"),
    }
}
