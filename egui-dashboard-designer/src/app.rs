use crate::components::{Dashboard, PropertyPanel, WidgetPanel};
use crate::widgets::{DashboardWidget, WidgetType};
use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct DashboardApp {
    dashboard: Dashboard,
    widget_panel: WidgetPanel,
    property_panel: PropertyPanel,
    selected_widget_id: Option<uuid::Uuid>,
}

impl DashboardApp {
    pub fn new() -> Self {
        Self {
            dashboard: Dashboard::new(),
            widget_panel: WidgetPanel::new(),
            property_panel: PropertyPanel::new(),
            selected_widget_id: None,
        }
    }

    fn handle_widget_selection(&mut self, widget_id: Option<uuid::Uuid>) {
        self.selected_widget_id = widget_id;
        self.property_panel.set_selected_widget(widget_id);
    }

    fn add_widget(&mut self, widget_type: WidgetType, position: egui::Pos2) {
        let widget = DashboardWidget::new(widget_type, position);
        let widget_id = widget.id;
        self.dashboard.add_widget(widget);
        self.handle_widget_selection(Some(widget_id));
    }
}

impl eframe::App for DashboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Dashboard").clicked() {
                        self.dashboard = Dashboard::new();
                        self.selected_widget_id = None;
                        ui.close_menu();
                    }
                    if ui.button("Save Dashboard").clicked() {
                        // TODO: Implement save functionality
                        ui.close_menu();
                    }
                    if ui.button("Load Dashboard").clicked() {
                        // TODO: Implement load functionality
                        ui.close_menu();
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Delete Selected").clicked() {
                        if let Some(widget_id) = self.selected_widget_id {
                            self.dashboard.remove_widget(widget_id);
                            self.selected_widget_id = None;
                        }
                        ui.close_menu();
                    }
                });
            });
        });

        // Left panel for widget palette
        egui::SidePanel::left("widget_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                if let Some(widget_type) = self.widget_panel.render(ui) {
                    // Add widget to center of dashboard
                    let center_pos = egui::pos2(400.0, 300.0);
                    self.add_widget(widget_type, center_pos);
                }
            });

        // Right panel for properties
        egui::SidePanel::right("property_panel")
            .resizable(true)
            .default_width(250.0)
            .show(ctx, |ui| {
                self.property_panel.render(ui, &mut self.dashboard);
            });

        // Central dashboard area
        egui::CentralPanel::default().show(ctx, |ui| {
            let selected_widget = self.dashboard.render(ui, self.selected_widget_id);
            if selected_widget != self.selected_widget_id {
                self.handle_widget_selection(selected_widget);
            }
        });
    }
}
