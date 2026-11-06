//! Tauri 窗口动作与核心库的可见性控制器对接。

use pcl_core::app::visibility::{self, LauncherVisibility, WindowActions};
use pcl_core::minecraft::launch::session::SessionEvent;

/// 借用包装，避免在应用 crate 里为外部类型实现外部 trait。
struct TauriWindow<'a>(&'a tauri::WebviewWindow);

impl WindowActions for TauriWindow<'_> {
    fn hide(&self) -> Result<(), String> {
        self.0.hide().map_err(|error| error.to_string())
    }

    fn show(&self) -> Result<(), String> {
        self.0.show().map_err(|error| error.to_string())
    }

    fn minimize(&self) -> Result<(), String> {
        self.0.minimize().map_err(|error| error.to_string())
    }

    fn exit(&self) {
        use tauri::Manager;
        self.0.app_handle().exit(0);
    }
}

pub fn handle(
    window: &tauri::WebviewWindow,
    id: &str,
    mode: LauncherVisibility,
    event: SessionEvent,
) {
    visibility::handle(&TauriWindow(window), id, mode, event);
}
