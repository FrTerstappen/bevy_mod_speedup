use bevy::prelude::*;

#[allow(unused_imports)]
use crate::SpeedupAdjustments;

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub enum UnattendedAdjustment {
    #[default]
    NotImplemented,
    Completed,
    Failed,
}

#[derive(Debug)]
pub struct UnattendedPlugin;

impl Plugin for UnattendedPlugin {
    fn build(
        &self,
        _app: &mut App,
    ) {
        #[cfg(target_os = "windows")]
        {
            _app.add_systems(PreStartup, adjust_unattended_windows)
        }
    }
}

#[cfg(target_os = "windows")]
fn adjust_unattended_windows(mut adjustments: ResMut<'_, SpeedupAdjustments>) {
    // TODO this is only for the current thread. Can we ensure that bevy does this in the main thread?
    #[allow(unsafe_code)]
    unsafe {
        use windows::Win32::System::Power::{
            SetThreadExecutionState,
            ES_CONTINUOUS,
            ES_DISPLAY_REQUIRED,
            ES_SYSTEM_REQUIRED,
        };

        let flags = ES_CONTINUOUS | ES_DISPLAY_REQUIRED | ES_SYSTEM_REQUIRED;
        let result = SetThreadExecutionState(flags);
        // TODO is this the right check? Windows api says fail = NULL but we only have a u32
        if result.0 == 0 {
            warn!("Unable to adjust thread execution state");
            adjustments.unattended = UnattendedAdjustment::Failed;
        } else {
            info!("Adjusted execution state to prevent suspension");
            adjustments.unattended = UnattendedAdjustment::Completed;
        }
    }

    return;
}
