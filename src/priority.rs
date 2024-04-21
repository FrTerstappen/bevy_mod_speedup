use bevy::prelude::*;

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub enum PriorityAdjustment {
    #[default]
    Unknown,
    Completed,
    Failed,
}

#[derive(Debug)]
pub struct PriorityPlugin;

impl Plugin for PriorityPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.register_type::<PriorityAdjustment>();
        app.insert_resource(PriorityAdjustment::Unknown);

        #[cfg(target_os = "windows")]
        {
            use windows::Win32::System::Threading::{
                GetCurrentProcess,
                SetPriorityClass,
                ABOVE_NORMAL_PRIORITY_CLASS,
            };

            let priority_class = ABOVE_NORMAL_PRIORITY_CLASS;

            let process = unsafe { GetCurrentProcess() };
            let result = unsafe { SetPriorityClass(process, priority_class) };
            match result {
                Ok(()) => {
                    debug!("Adjusted process priority to above normal");
                    app.insert_resource(PriorityAdjustment::Completed);
                },
                Err(error) => {
                    warn!("Failed to adjust process priority. Error: {error}");
                    app.insert_resource(PriorityAdjustment::Failed);
                },
            }
        }

        #[cfg(target_os = "linux")]
        {
            // TODO check if this works on more platforms

            // Check if running as root
            // Root needed to increase priority
            match nix::unistd::getresuid() {
                Ok(uid) => {
                    if uid.real.is_root() || uid.effective.is_root() || uid.saved.is_root() {
                        info!("User is root. Trying to adjust priority");
                    } else {
                        warn!("Not running as root. No adjustment of priority possible");
                        app.insert_resource(PriorityAdjustment::Failed);
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
                    app.insert_resource(PriorityAdjustment::Completed);
                },
                _ => {
                    warn!("Adjusting priority failed");
                    app.insert_resource(PriorityAdjustment::Failed);
                },
            }
        }
    }
}
