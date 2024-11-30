use bevy::{
    prelude::*,
    window::PrimaryWindow,
};
use bevy_egui::{
    EguiContext,
    EguiPlugin,
    egui,
};
use bevy_mod_speedup::SpeedupAdjustments;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(EguiPlugin);
    app.add_plugins(bevy_mod_speedup::SpeedupPlugin);
    app.add_systems(Update, display_adjustment_status);
    app.run();
}

fn display_adjustment_status(
    mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
    speedup_adjustments: Res<'_, SpeedupAdjustments>,
) {
    egui::Window::new("Example window").show(egui_ctx.single_mut().get_mut(), |ui| {
        bevy_mod_speedup::egui::ui_for_adjustment(ui, &speedup_adjustments, true);
    });
}
