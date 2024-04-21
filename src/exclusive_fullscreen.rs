use bevy::prelude::*;

#[derive(Debug, Default, Resource, Reflect)]
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
        app: &mut App,
    ) {
        app.register_type::<ExclusiveFullscreenAdjustment>();
        app.insert_resource(ExclusiveFullscreenAdjustment::NotSupportingStatus);

        #[cfg(target_os = "windows")]
        {
            use windows::Win32::UI::HiDpi::{
                AreDpiAwarenessContextsEqual,
                GetDpiAwarenessContextForProcess,
                IsValidDpiAwarenessContext,
                SetProcessDpiAwarenessContext,
                DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
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
    }
}
