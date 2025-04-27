use bevy::prelude::*;

#[allow(unused_imports)]
use crate::SpeedupAdjustments;

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub enum PriorityAdjustment {
    #[default]
    NotImplemented,
    Completed,
    Failed,
}

#[derive(Debug)]
pub struct PriorityPlugin;

impl Plugin for PriorityPlugin {
    fn build(
        &self,
        _app: &mut App,
    ) {
        #[cfg(target_os = "windows")]
        {
            _app.add_systems(PreStartup, adjust_priority_windows);
        }

        #[cfg(target_os = "linux")]
        {
            _app.add_systems(PreStartup, adjust_priority_linux);
        }
    }
}

#[cfg(target_os = "windows")]
fn adjust_priority_windows(mut adjustments: ResMut<'_, SpeedupAdjustments>) {
    use windows::Win32::System::Threading::{
        ABOVE_NORMAL_PRIORITY_CLASS,
        GetCurrentProcess,
        SetPriorityClass,
    };

    let priority_class = ABOVE_NORMAL_PRIORITY_CLASS;

    let process = unsafe { GetCurrentProcess() };
    let result = unsafe { SetPriorityClass(process, priority_class) };
    match result {
        Ok(()) => {
            debug!("Adjusted process priority to above normal");
            adjustments.priority = PriorityAdjustment::Completed;
        },
        Err(error) => {
            warn!("Failed to adjust process priority. Error: {error}");
            adjustments.priority = PriorityAdjustment::Failed;
        },
    }
}

#[cfg(target_os = "linux")]
fn adjust_priority_linux(mut adjustments: ResMut<'_, SpeedupAdjustments>) {
    // TODO check if this works on more platforms

    // Check if running as root
    // Root needed to increase priority
    match nix::unistd::getresuid() {
        Ok(uid) => {
            if uid.real.is_root() || uid.effective.is_root() || uid.saved.is_root() {
                info!("User is root. Trying to adjust priority");
            } else {
                warn!("Not running as root. No adjustment of priority possible");
                adjustments.priority = PriorityAdjustment::Failed;
                return;
            }
        },
        Err(e) => {
            warn!("Unable to get uid/gid: {e}. Still trying to adjust priority");
        },
    }

    let which = nix::libc::PRIO_PROCESS;
    let who = 0; // Current process
    let priority = -5; // -20 .. 19; -20 is highest priority; 0 is default
    let result = unsafe { nix::libc::setpriority(which, who, priority) };
    match result {
        0 => {
            info!("Adjusted priority");
            adjustments.priority = PriorityAdjustment::Completed;
        },
        _ => {
            warn!("Adjusting priority failed");
            adjustments.priority = PriorityAdjustment::Failed;
        },
    }
}
