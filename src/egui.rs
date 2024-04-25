use crate::SpeedupAdjustments;

pub fn ui_for_adjustment(
    ui: &mut egui::Ui,
    _speedup_adjustments: &SpeedupAdjustments,
    open_by_default: bool,
) {
    egui::CollapsingHeader::new("Speedup adjustments").default_open(open_by_default).show(ui, |ui| {
        ui.columns(2, |columns| {
            #[cfg(feature = "exclusive_fullscreen")]
            {
                columns[0].label("Exclusive fullscreen");
                columns[1].label(format!("{:?}", _speedup_adjustments.exclusive_fullscreen));
            }

            #[cfg(feature = "power")]
            {
                columns[0].label("Power");
                columns[1].label(format!("{:?}", _speedup_adjustments.power));
            }

            #[cfg(feature = "priority")]
            {
                columns[0].label("Priority");
                columns[1].label(format!("{:?}", _speedup_adjustments.priority));
            }

            #[cfg(feature = "request_fast_gpu")]
            {
                columns[0].label("Request fast GPU");
                columns[1].label(format!("{:?}", _speedup_adjustments.request_fast_gpu));
            }

            #[cfg(feature = "unattended")]
            {
                columns[0].label("Unattended");
                columns[1].label(format!("{:?}", _speedup_adjustments.unattended));
            }
        });
    });
}
