#[macro_use]
extern crate core;

use core::plugins::runtime::*;

#[derive(Debug, Default)]
struct ContainerdRuntimePlugin;

declare_plugin!(ContainerdRuntimePlugin, ContainerdRuntimePlugin::default);

impl RuntimePlugin for ContainerdRuntimePlugin {
    /// The name of the plugin used to identify it.
    fn name(&self) -> String {
        return "Boby Jones".to_string();
    }
    /// A callback fired immediately after the plugin is loaded. Usually used
    /// for initialization.
    fn on_plugin_load(&self) {}
    /// A callback fired immediately before the plugin is unloaded. Use this if
    /// you need to do any cleanup.
    fn on_plugin_unload(&self) {}
    fn get_features(&self) -> Vec<RuntimeFeatures> {
        return vec![RuntimeFeatures::WorkloadRunner, RuntimeFeatures::Container];
    }
    fn get_version(&self) -> i32 {
        0
    }

    //Required for feature WorkloadRunner
    fn create_workload(
        &self,
        id: String,
        config: &RuntimeConfig,
        options: &Option<SandboxConfig>,
    ) -> Result<String, RuntimeError> {
        return Err(RuntimeError::new(RuntimeErrorType::Unknown));
    }

    fn start_workload(&self, id: String) -> Option<RuntimeError> {
        return None;
    }

    fn stop_workload(&self, id: String, timeout: i32) -> Option<RuntimeError> {
        return None;
    }

    fn remove_workload(&self, id: String) -> Option<RuntimeError> {
        return None;
    }

    fn status_workload(&self, id: String) -> Result<WorkloadStatus, RuntimeError> {
        return Err(RuntimeError::new(RuntimeErrorType::Unknown));
    }

    //Required for feature container && vm
    fn update_workload_resources(
        &self,
        id: String,
        rez: WorkloadResources,
    ) -> Option<RuntimeError> {
        return None;
    }

    fn exec_sync(
        &self,
        id: String,
        cmd: &[String],
        timeout: i32,
    ) -> (&[u8], &[u8], Option<RuntimeError>) {
        return (&[0], &[0], None);
    }
}
