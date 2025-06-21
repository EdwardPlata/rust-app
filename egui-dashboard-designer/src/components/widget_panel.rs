use crate::widgets::WidgetType;
use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct WidgetPanel {
    search_filter: String,
}

impl WidgetPanel {
    pub fn new() -> Self {
        Self {
            search_filter: String::new(),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) -> Option<WidgetType> {
        let mut selected_widget = None;

        ui.heading("📦 Widget Palette");
        ui.separator();

        // Search filter
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.search_filter);
        });
        ui.separator();

        ui.label("Click to add widget:");

        // Widget buttons
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for widget_type in WidgetType::all() {
                    let widget_name = widget_type.name();

                    // Apply search filter
                    if !self.search_filter.is_empty()
                        && !widget_name
                            .to_lowercase()
                            .contains(&self.search_filter.to_lowercase())
                    {
                        continue;
                    }

                    let icon = match widget_type {
                        WidgetType::Chart => "📈",
                        WidgetType::Gauge => "⏲️",
                        WidgetType::Table => "📋",
                        WidgetType::Text => "📝",
                        WidgetType::Button => "🔘",
                        WidgetType::Image => "🖼️",
                    };

                    if ui
                        .add_sized(
                            [ui.available_width(), 40.0],
                            egui::Button::new(format!("{} {}", icon, widget_name))
                                .fill(egui::Color32::from_rgb(230, 230, 255)),
                        )
                        .clicked()
                    {
                        selected_widget = Some(widget_type);
                    }

                    ui.add_space(4.0);
                }
            });

        ui.separator();

        // Instructions
        ui.group(|ui| {
            ui.label("💡 Instructions:");
            ui.label("• Click a widget to add it");
            ui.label("• Drag widgets to move");
            ui.label("• Click to select");
            ui.label("• Use properties panel to edit");
        });

        selected_widget
    }
}
