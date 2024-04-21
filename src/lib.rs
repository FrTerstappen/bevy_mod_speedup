#[cfg(feature = "exclusive_fullscreen")]
pub mod exclusive_fullscreen;

#[cfg(feature = "power")]
pub mod power;

#[cfg(feature = "request_fast_gpu")]
pub mod request_fast_gpu;

#[cfg(feature = "priority")]
pub mod priority;

#[cfg(feature = "unattended")]
pub mod unattended;

use bevy::prelude::*;

#[derive(Debug, SystemSet, Clone, PartialEq, Eq, Hash)]
pub enum SpeedupSet {
    PreStartup,
    Startup,
    PostStartup,
}

#[derive(Debug)]
pub struct SpeedupPlugin;

impl Plugin for SpeedupPlugin {
    fn build(
        &self,
        _app: &mut App,
    ) {
        #[cfg(feature = "exclusive_fullscreen")]
        _app.add_plugins(exclusive_fullscreen::ExclusiveFullscreenPlugin);

        #[cfg(feature = "power")]
        _app.add_plugins(power::PowerPlugin);

        #[cfg(feature = "request_fast_gpu")]
        _app.add_plugins(request_fast_gpu::RequestFastGpuPlugin);

        #[cfg(feature = "priority")]
        _app.add_plugins(priority::PriorityPlugin);

        #[cfg(feature = "unattended")]
        _app.add_plugins(unattended::UnattendedPlugin);
    }
}
