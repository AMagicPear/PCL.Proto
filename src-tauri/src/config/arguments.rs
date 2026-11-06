//! 启动参数设置的读写；合并规则在 `pcl-core`。

use std::path::Path;

use pcl_core::minecraft::java::preference::current_instance;
use pcl_core::minecraft::launch::arguments::LaunchArguments;

use crate::config::states::AppState;

pub fn update(
    state: &mut AppState,
    instance_directory: Option<&Path>,
    settings: LaunchArguments,
) -> Result<LaunchArguments, String> {
    settings.validate(instance_directory.is_some())?;
    if let Some(path) = instance_directory {
        current_instance(state.active_game_instance.as_deref(), path)
            .map_err(|error| error.to_string())?;
        state
            .instance_launch_arguments
            .insert(path.to_owned(), settings.clone());
    } else {
        state.launch_arguments = Some(settings.clone());
    }
    Ok(settings)
}
