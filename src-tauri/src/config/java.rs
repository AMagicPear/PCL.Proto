//! Java 列表的扫描编排与持久化；扫描与选择规则在 `pcl-core`。

use std::path::{Path, PathBuf};

use pcl_core::minecraft::instance::McInstance;
use pcl_core::minecraft::java::preference::JavaLaunchInfo;
use pcl_core::minecraft::java::{JavaEntry, JavaRoots, JavaState, preference::current_instance};
use pcl_core::runtime::RuntimePaths;

use crate::config::manager::ConfigManager;
use crate::config::states::AppState;

/// 当前活动实例必须与预期一致，避免异步操作落到别的实例上。
pub fn active_instance<'a>(state: &'a AppState, expected: &Path) -> Result<&'a McInstance, String> {
    current_instance(state.active_game_instance.as_deref(), expected)
        .map_err(|error| error.to_string())
}

pub fn state() -> JavaState {
    let state = ConfigManager::instance().app_state.blocking_lock();
    JavaState {
        runtimes: state.java_runtimes.clone(),
        selected: state.launcher_setup.selected_java.clone(),
    }
}

pub fn paths() -> RuntimePaths {
    let config = ConfigManager::instance();
    RuntimePaths {
        config_dir: config.config_dir().to_path_buf(),
        java_runtime_dir: config.java_runtime_dir(),
    }
}

/// 重新扫描 Java 列表；结果写回配置。
pub async fn refresh(force: bool) -> Result<Vec<JavaEntry>, String> {
    let config = ConfigManager::instance();
    let (known, repositories, selected) = {
        let state = config.app_state.lock().await;
        (
            state.java_runtimes.clone(),
            state
                .repositories
                .iter()
                .map(|repository| repository.path.clone())
                .collect::<Vec<_>>(),
            state.launcher_setup.selected_java.clone(),
        )
    };
    let scanned = pcl_core::minecraft::java::refresh(
        force,
        &JavaState {
            runtimes: known,
            selected,
        },
        &JavaRoots {
            repositories,
            runtime_dir: config.java_runtime_dir(),
        },
    )
    .await?;
    let runtimes = scanned.runtimes.clone();
    let selected = scanned.selected.clone();
    tokio::task::spawn_blocking(move || {
        config.update(|state| {
            state.java_runtimes = runtimes.clone();
            state.launcher_setup.selected_java = selected.clone();
            Ok(runtimes.clone())
        })
    })
    .await
    .map_err(|error| error.to_string())?
}

/// 组装某个实例的 Java 启动信息；缺少运行时时会先刷新一次。
pub async fn prepare(expected: PathBuf) -> Result<JavaLaunchInfo, String> {
    async fn evaluate(expected: PathBuf) -> Result<JavaLaunchInfo, String> {
        let config = ConfigManager::instance();
        let (instance, runtimes, global, settings) = {
            let state = config.app_state.lock().await;
            let instance = active_instance(&state, &expected)
                .map_err(|e| e.to_string())?
                .clone();
            let settings = state
                .java_preferences
                .get(&instance.directory)
                .cloned()
                .unwrap_or_default();
            (
                instance,
                state.java_runtimes.clone(),
                state.launcher_setup.selected_java.clone(),
                settings,
            )
        };
        let result = tokio::task::spawn_blocking(move || {
            let instance = instance
                .refresh()
                .map_err(|e| format!("Cannot refresh Minecraft instance: {e:?}"))?;
            JavaLaunchInfo::for_instance(&instance, &runtimes, global.as_deref(), &settings)
                .map_err(|e| e.to_string())
        })
        .await
        .map_err(|e| e.to_string())?;
        active_instance(&*config.app_state.lock().await, &expected).map_err(|e| e.to_string())?;
        result
    }
    let mut info = evaluate(expected.clone()).await?;
    if info.runtime.is_none() {
        refresh(false).await?;
        info = evaluate(expected).await?;
    }
    Ok(info)
}
