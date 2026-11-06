//! 实例启动附加设置：读写配置并折算成 `pcl-core` 的启动参数。

use std::path::Path;

use pcl_core::minecraft::java::preference::current_instance;
use pcl_core::minecraft::launch::settings::{
    GlobalLaunchSwitches, InstanceLaunchSettings, LaunchExtras, resolve,
};

use crate::config::states::AppState;

pub fn update(
    state: &mut AppState,
    instance_directory: &Path,
    settings: InstanceLaunchSettings,
) -> Result<InstanceLaunchSettings, String> {
    settings.validate()?;
    current_instance(state.active_game_instance.as_deref(), instance_directory)
        .map_err(|error| error.to_string())?;
    if settings == InstanceLaunchSettings::default() {
        state.instance_launch_settings.remove(instance_directory);
    } else {
        state
            .instance_launch_settings
            .insert(instance_directory.to_owned(), settings.clone());
    }
    Ok(settings)
}

/// 把全局开关和实例覆盖折算成实际生效的启动选项。
pub fn resolve_for(state: &AppState, instance: Option<&Path>) -> LaunchExtras {
    let global = GlobalLaunchSwitches {
        version_info: state.version_info.clone(),
        renderer: state.renderer,
        disable_jlw: state.disable_jlw,
        disable_legacy_fix: state.disable_legacy_fix,
        disable_lwjgl_unsafe_agent: state.disable_lwjgl_unsafe_agent,
    };
    let instance_settings = instance.and_then(|path| state.instance_launch_settings.get(path));
    resolve(&global, instance_settings)
}
