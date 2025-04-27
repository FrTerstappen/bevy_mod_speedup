use bevy::prelude::*;

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub enum RequestFastGpuAdjustment {
    #[default]
    NotSupportingStatus,
}

/// Request for usage of high performance gpu in multi gpu setups.
///
/// # Platform behavior:
///
/// ## Windows:
/// * Adds the variables `NvOptimusEnablement` and `AmdPowerXpressRequestHighPerformance` with a value of one.
/// * Required the user to also export those by setting the linker args. This can be done in build.rs via:
/// ```rust
/// println!("cargo:rustc-link-arg=/EXPORT:NvOptimusEnablement");
/// println!("cargo:rustc-link-arg=/EXPORT:AmdPowerXpressRequestHighPerformance");
/// ```
///
/// ## Other platforms:
///
/// Currently does nothing on other platforms.
///
/// # Validating
///
/// Check with dumpbin /exports $APPLICATION.exe
#[macro_export]
macro_rules! request_fast_gpu {
    () => {
        #[cfg(target_os = "windows")]
        #[no_mangle]
        pub static NvOptimusEnablement: i32 = 1;

        #[cfg(target_os = "windows")]
        #[no_mangle]
        pub static AmdPowerXpressRequestHighPerformance: i32 = 1;
    };
}

#[derive(Debug)]
pub struct RequestFastGpuPlugin;

impl Plugin for RequestFastGpuPlugin {
    fn build(
        &self,
        _app: &mut App,
    ) {
    }
}
