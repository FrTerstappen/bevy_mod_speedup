use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    EguiPlugin,
    egui,
};
use bevy_mod_speedup::SpeedupAdjustments;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(EguiPlugin {
        enable_multipass_for_primary_context: true
    });
    app.add_plugins(bevy_mod_speedup::SpeedupPlugin);
    app.add_systems(Update, display_adjustment_status);
    app.run();
}

fn display_adjustment_status(
    mut contexts: EguiContexts,
    speedup_adjustments: Res<'_, SpeedupAdjustments>,
) {
    egui::Window::new("Example window").show(contexts.ctx_mut(), |ui| {
        bevy_mod_speedup::egui::ui_for_adjustment(ui, &speedup_adjustments, true);
    });
}
