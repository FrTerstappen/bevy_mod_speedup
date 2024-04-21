use bevy::prelude::*;

#[cfg(target_os = "windows")]
#[derive(Resource, Deref, DerefMut)]
struct OriginalPowerScheme(windows::core::GUID);

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub enum PowerAdjustment {
    #[default]
    Unknown,
    Completed,
    Failed,
    NoAdjustmentNeeded,
}

#[derive(Debug)]
pub struct PowerPlugin;

impl Plugin for PowerPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.register_type::<PowerAdjustment>();
        app.insert_resource(PowerAdjustment::Unknown);

        // TODO decide if re-activating power saving on lost focus is possible/preferable

        #[cfg(target_os = "windows")]
        {
            use windows::{
                core::GUID,
                Win32::System::{
                    Power,
                    SystemServices::GUID_MIN_POWER_SAVINGS,
                },
            };

            // Determine currently active power scheme
            let mut power_scheme: *mut GUID = std::ptr::null_mut();
            let result = unsafe { Power::PowerGetActiveScheme(None, &mut power_scheme) };

            if result.is_err() {
                warn!("Unable to determine original power saving scheme. Not adjusting it");
                warn!("Error: {result:?}");
                app.insert_resource(PowerAdjustment::Failed);
                return;
            }

            let power_scheme = unsafe { power_scheme.as_ref() };
            let Some(power_scheme) = power_scheme else {
                warn!("Unable to determine original power saving scheme. Got no value");
                app.insert_resource(PowerAdjustment::Failed);
                return;
            };

            const TARGET_SCHEME: &GUID = &GUID_MIN_POWER_SAVINGS;
            if power_scheme == TARGET_SCHEME {
                info!("Power scheme is already set. No adjustment needed");
                app.insert_resource(PowerAdjustment::NoAdjustmentNeeded);
                return;
            }

            let scheme = OriginalPowerScheme(*power_scheme);
            app.insert_resource(scheme);

            // Adjust power scheme
            let result = unsafe { Power::PowerSetActiveScheme(None, Some(TARGET_SCHEME)) };
            if result.is_err() {
                warn!("Unable to adjust power scheme");
                app.insert_resource(PowerAdjustment::Failed);
                return;
            }

            app.insert_resource(PowerAdjustment::Completed);

            // Restore on exit (if possible)
            app.add_systems(PostUpdate, restore_on_exit_windows);
            return;
        }
    }
}

#[cfg(target_os = "windows")]
fn restore_on_exit_windows(
    mut events: EventReader<AppExit>,
    #[cfg(target_os = "windows")] original_scheme: Res<OriginalPowerScheme>,
) {
    if events.is_empty() {
        return;
    }

    events.clear();

    use windows::Win32::System::Power;

    // Adjust power scheme
    let result = unsafe { Power::PowerSetActiveScheme(None, Some(&**original_scheme)) };
    if result.is_err() {
        warn!("Unable to reset power scheme on exit");
    }
}
