use bevy::prelude::*;

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub enum ExclusiveFullscreenAdjustment {
    #[default]
    NotSupportingStatus,
}

#[derive(Debug)]
pub struct ExclusiveFullscreenPlugin;

impl Plugin for ExclusiveFullscreenPlugin {
    fn build(
        &self,
        _app: &mut App,
    ) {
        #[cfg(target_os = "windows")]
        {
            _app.add_systems(PreStartup, enable_dpi_awareness_windows);
        }
    }
}
#[cfg(target_os = "windows")]
fn enable_dpi_awareness_windows() {
    use windows::Win32::UI::HiDpi::{
        AreDpiAwarenessContextsEqual,
        DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
        GetDpiAwarenessContextForProcess,
        IsValidDpiAwarenessContext,
        SetProcessDpiAwarenessContext,
    };

    let target = DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2;

    // Check if this is valid for the system
    let result = unsafe { IsValidDpiAwarenessContext(target) };
    if !result.as_bool() {
        info!("The target dpi awareness if not valid for this system. No adjustment possible");
        return;
    }

    // Check if this is already set (probably from application manifest)
    let current = unsafe { GetDpiAwarenessContextForProcess(None) };
    let result = unsafe { AreDpiAwarenessContextsEqual(current, target) };
    if result.as_bool() {
        info!("Process is already DPI aware. No adjustment needed");
        return;
    }

    // Adjust dpi awareness
    let result = unsafe { SetProcessDpiAwarenessContext(target) };
    match result {
        Ok(_) => info!("Adjusted DPI awareness"),
        Err(error) => {
            warn!("Adjusting DPI awareness failed. Error: {error}");
        },
    }
}
