use bevy::prelude::*;

#[allow(unused_imports)]
use crate::SpeedupAdjustments;

#[cfg(target_os = "windows")]
#[derive(Resource, Deref, DerefMut)]
struct OriginalPowerScheme(windows::core::GUID);

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub enum PowerAdjustment {
    #[default]
    NotImplemented,
    Completed,
    Failed,
    Restored,
    NoAdjustmentNeeded,
}

#[derive(Debug)]
pub struct PowerPlugin;

impl Plugin for PowerPlugin {
    fn build(
        &self,
        _app: &mut App,
    ) {
        // TODO decide if re-activating power saving on lost focus is possible/preferable

        #[cfg(target_os = "windows")]
        {
            _app.add_systems(PreStartup, adjust_power_windows);
            _app.add_systems(PostUpdate, restore_on_exit_windows);
        }
    }
}

#[cfg(target_os = "windows")]
fn adjust_power_windows(mut adjustments: ResMut<'_, SpeedupAdjustments>) {
    use windows::{
        Win32::System::{
            Power,
            SystemServices::GUID_MIN_POWER_SAVINGS,
        },
        core::GUID,
    };

    // Determine currently active power scheme
    let mut power_scheme: *mut GUID = std::ptr::null_mut();
    let result = unsafe { Power::PowerGetActiveScheme(None, &mut power_scheme) };

    if result.is_err() {
        adjustments.power = PowerAdjustment::Failed;
        return;
    }
    match result {
        Ok(()) => {},
        Err(error) => {
            warn!("Unable to determine original power saving scheme. Error: {error}");
            warn!("Not adjusting power saving scheme.");
            adjustments.power = PowerAdjustment::Failed;
            return;
        },
    }

    let power_scheme = unsafe { power_scheme.as_ref() };
    let Some(power_scheme) = power_scheme else {
        warn!("Unable to determine original power saving scheme. Got no value");
        adjustments.power = PowerAdjustment::Failed;
        return;
    };

    const TARGET_SCHEME: &GUID = &GUID_MIN_POWER_SAVINGS;
    if power_scheme == TARGET_SCHEME {
        info!("Power scheme is already set. No adjustment needed");
        adjustments.power = PowerAdjustment::NoAdjustmentNeeded;
        return;
    }

    let scheme = OriginalPowerScheme(*power_scheme);
    app.insert_resource(scheme);

    // Adjust power scheme
    let result = unsafe { Power::PowerSetActiveScheme(None, Some(TARGET_SCHEME)) };
    match result {
        Ok(()) => {},
        Err(error) => {
            warn!("Unable to adjust power scheme. Error: {error}");
            adjustments.power = PowerAdjustment::Failed;
            return;
        },
    }

    adjustments.power = PowerAdjustment::Completed;
}

#[cfg(target_os = "windows")]
fn restore_on_exit_windows(
    mut adjustments: ResMut<'_, SpeedupAdjustments>,
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
    match result {
        Ok(()) => {
            adjustments.power = PowerAdjustment::Restored;
        },
        Err(error) => {
            warn!("Unable to reset power scheme on exit. Error: {error}");
        },
    }
}
