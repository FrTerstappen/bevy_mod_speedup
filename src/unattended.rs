use bevy::prelude::*;

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub enum UnattendedAdjustment {
    #[default]
    Unknown,
    Completed,
    Failed,
    NoAdjustmentNeeded,
}

#[derive(Debug)]
pub struct UnattendedPlugin;

impl Plugin for UnattendedPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.register_type::<UnattendedAdjustment>();
        app.insert_resource(UnattendedAdjustment::Unknown);

        #[cfg(target_os = "windows")]
        {
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
                    app.insert_resource(UnattendedAdjustment::Failed);
                } else {
                    info!("Adjusted execution state to prevent suspension");
                    app.insert_resource(UnattendedAdjustment::Completed);
                }
            }

            return;
        }
    }
}
