#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "egui")]
pub mod egui;

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

#[derive(SystemSet, Clone, Debug, Eq, Hash, PartialEq)]
pub enum SpeedupSet {
    PreStartup,
    Startup,
    PostStartup,
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct SpeedupAdjustments {
    #[cfg(feature = "exclusive_fullscreen")]
    exclusive_fullscreen: crate::exclusive_fullscreen::ExclusiveFullscreenAdjustment,
    #[cfg(feature = "power")]
    power:                crate::power::PowerAdjustment,
    #[cfg(feature = "priority")]
    priority:             crate::priority::PriorityAdjustment,
    #[cfg(feature = "request_fast_gpu")]
    request_fast_gpu:     crate::request_fast_gpu::RequestFastGpuAdjustment,
    #[cfg(feature = "unattended")]
    unattended:           crate::unattended::UnattendedAdjustment,
}

#[derive(Debug)]
pub struct SpeedupPlugin;

impl Plugin for SpeedupPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.register_type::<SpeedupAdjustments>();
        app.init_resource::<SpeedupAdjustments>();

        #[cfg(feature = "exclusive_fullscreen")]
        app.add_plugins(exclusive_fullscreen::ExclusiveFullscreenPlugin);

        #[cfg(feature = "power")]
        app.add_plugins(power::PowerPlugin);

        #[cfg(feature = "request_fast_gpu")]
        app.add_plugins(request_fast_gpu::RequestFastGpuPlugin);

        #[cfg(feature = "priority")]
        app.add_plugins(priority::PriorityPlugin);

        #[cfg(feature = "unattended")]
        app.add_plugins(unattended::UnattendedPlugin);
    }
}
