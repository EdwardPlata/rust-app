use crate::widgets::{DashboardWidget, WidgetData};
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default, Serialize, Deserialize)]
pub struct Dashboard {
    widgets: HashMap<Uuid, DashboardWidget>,
    grid_size: f32,
    snap_to_grid: bool,
    background_color: egui::Color32,
}

impl Dashboard {
    pub fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            grid_size: 20.0,
            snap_to_grid: true,
            background_color: egui::Color32::from_rgb(240, 240, 240),
        }
    }

    pub fn add_widget(&mut self, widget: DashboardWidget) {
        self.widgets.insert(widget.id, widget);
    }

    pub fn remove_widget(&mut self, widget_id: Uuid) {
        self.widgets.remove(&widget_id);
    }

    pub fn get_widget_mut(&mut self, widget_id: Uuid) -> Option<&mut DashboardWidget> {
        self.widgets.get_mut(&widget_id)
    }

    pub fn get_widget(&self, widget_id: Uuid) -> Option<&DashboardWidget> {
        self.widgets.get(&widget_id)
    }

    fn snap_position(&self, pos: egui::Pos2) -> egui::Pos2 {
        if self.snap_to_grid {
            egui::pos2(
                (pos.x / self.grid_size).round() * self.grid_size,
                (pos.y / self.grid_size).round() * self.grid_size,
            )
        } else {
            pos
        }
    }

    fn draw_grid(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        if !self.snap_to_grid {
            return;
        }

        let painter = ui.painter();
        let grid_color = egui::Color32::from_rgba_premultiplied(200, 200, 200, 100);

        // Draw vertical lines
        let mut x = rect.min.x;
        while x <= rect.max.x {
            painter.line_segment(
                [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                egui::Stroke::new(1.0, grid_color),
            );
            x += self.grid_size;
        }

        // Draw horizontal lines
        let mut y = rect.min.y;
        while y <= rect.max.y {
            painter.line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                egui::Stroke::new(1.0, grid_color),
            );
            y += self.grid_size;
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, selected_widget_id: Option<Uuid>) -> Option<Uuid> {
        let available_rect = ui.available_rect_before_wrap();

        // Draw background
        ui.painter()
            .rect_filled(available_rect, egui::Rounding::ZERO, self.background_color);

        // Draw grid
        self.draw_grid(ui, available_rect);

        let mut new_selection = selected_widget_id;

        // Handle clicks on empty space (deselect)
        if ui
            .allocate_rect(available_rect, egui::Sense::click())
            .clicked()
        {
            new_selection = None;
        }

        // Render widgets
        let widget_ids: Vec<Uuid> = self.widgets.keys().copied().collect();
        for widget_id in widget_ids {
            if let Some(widget) = self.widgets.get_mut(&widget_id) {
                let is_selected = selected_widget_id == Some(widget_id);
                let response = widget.render(ui, is_selected);

                if response.clicked() {
                    new_selection = Some(widget_id);
                }

                // Snap to grid if enabled
                if response.drag_released() && self.snap_to_grid {
                    widget.position = self.snap_position(widget.position);
                }
            }
        }

        // Show dashboard info
        ui.allocate_ui_at_rect(
            egui::Rect::from_min_size(
                available_rect.min + egui::vec2(10.0, 10.0),
                egui::vec2(200.0, 50.0),
            ),
            |ui| {
                ui.group(|ui| {
                    ui.label(format!("Widgets: {}", self.widgets.len()));
                    ui.label(format!(
                        "Grid: {}",
                        if self.snap_to_grid { "On" } else { "Off" }
                    ));
                });
            },
        );

        new_selection
    }
}
