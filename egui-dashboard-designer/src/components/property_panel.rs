use crate::components::Dashboard;
use crate::widgets::{ChartType, WidgetData};
use eframe::egui;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Serialize, Deserialize)]
pub struct PropertyPanel {
    selected_widget_id: Option<Uuid>,
}

impl PropertyPanel {
    pub fn new() -> Self {
        Self {
            selected_widget_id: None,
        }
    }

    pub fn set_selected_widget(&mut self, widget_id: Option<Uuid>) {
        self.selected_widget_id = widget_id;
    }

    pub fn render(&mut self, ui: &mut egui::Ui, dashboard: &mut Dashboard) {
        ui.heading("⚙️ Properties");
        ui.separator();

        if let Some(widget_id) = self.selected_widget_id {
            if let Some(widget) = dashboard.get_widget_mut(widget_id) {
                ui.group(|ui| {
                    ui.label(format!("📦 {}", widget.widget_type.name()));
                    ui.label(format!("ID: {}", widget.id));
                });

                ui.separator();

                // General properties
                ui.collapsing("General", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Title:");
                        ui.text_edit_singleline(&mut widget.title);
                    });

                    ui.horizontal(|ui| {
                        ui.label("X:");
                        ui.add(egui::DragValue::new(&mut widget.position.x).speed(1.0));
                        ui.label("Y:");
                        ui.add(egui::DragValue::new(&mut widget.position.y).speed(1.0));
                    });

                    ui.horizontal(|ui| {
                        ui.label("Width:");
                        ui.add(
                            egui::DragValue::new(&mut widget.size.x)
                                .speed(1.0)
                                .clamp_range(50.0..=1000.0),
                        );
                        ui.label("Height:");
                        ui.add(
                            egui::DragValue::new(&mut widget.size.y)
                                .speed(1.0)
                                .clamp_range(50.0..=800.0),
                        );
                    });
                });

                ui.separator();

                // Widget-specific properties
                ui.collapsing("Widget Data", |ui| match &mut widget.data {
                    WidgetData::Chart {
                        data_points,
                        chart_type,
                    } => {
                        self.render_chart_properties(ui, data_points, chart_type);
                    }
                    WidgetData::Gauge {
                        value,
                        min,
                        max,
                        unit,
                    } => {
                        self.render_gauge_properties(ui, value, min, max, unit);
                    }
                    WidgetData::Table { headers, rows } => {
                        self.render_table_properties(ui, headers, rows);
                    }
                    WidgetData::Text { content, font_size } => {
                        self.render_text_properties(ui, content, font_size);
                    }
                    WidgetData::Button { label, action } => {
                        self.render_button_properties(ui, label, action);
                    }
                    WidgetData::Image { path, scale } => {
                        self.render_image_properties(ui, path, scale);
                    }
                });
            } else {
                ui.label("⚠️ Selected widget not found");
            }
        } else {
            ui.group(|ui| {
                ui.label("ℹ️ No widget selected");
                ui.label("Click on a widget to edit its properties");
            });
        }

        ui.separator();

        // Dashboard settings
        ui.collapsing("🏠 Dashboard Settings", |ui| {
            ui.label("Grid settings and other dashboard options coming soon...");
        });
    }

    fn render_chart_properties(
        &self,
        ui: &mut egui::Ui,
        data_points: &mut Vec<(f64, f64)>,
        chart_type: &mut ChartType,
    ) {
        ui.horizontal(|ui| {
            ui.label("Chart Type:");
            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", chart_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(chart_type, ChartType::Line, "Line");
                    ui.selectable_value(chart_type, ChartType::Bar, "Bar");
                    ui.selectable_value(chart_type, ChartType::Scatter, "Scatter");
                });
        });

        ui.label("Data Points:");
        egui::ScrollArea::vertical()
            .max_height(150.0)
            .show(ui, |ui| {
                let mut to_remove = None;
                for (i, (x, y)) in data_points.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", i + 1));
                        ui.add(egui::DragValue::new(x).speed(0.1).prefix("X: "));
                        ui.add(egui::DragValue::new(y).speed(0.1).prefix("Y: "));
                        if ui.button("❌").clicked() {
                            to_remove = Some(i);
                        }
                    });
                }
                if let Some(index) = to_remove {
                    data_points.remove(index);
                }
            });

        if ui.button("➕ Add Point").clicked() {
            data_points.push((data_points.len() as f64, 0.0));
        }
    }

    fn render_gauge_properties(
        &self,
        ui: &mut egui::Ui,
        value: &mut f64,
        min: &mut f64,
        max: &mut f64,
        unit: &mut String,
    ) {
        ui.horizontal(|ui| {
            ui.label("Value:");
            ui.add(
                egui::DragValue::new(value)
                    .speed(1.0)
                    .clamp_range(*min..=*max),
            );
        });

        ui.horizontal(|ui| {
            ui.label("Min:");
            ui.add(egui::DragValue::new(min).speed(1.0));
            ui.label("Max:");
            ui.add(egui::DragValue::new(max).speed(1.0));
        });

        ui.horizontal(|ui| {
            ui.label("Unit:");
            ui.text_edit_singleline(unit);
        });
    }

    fn render_table_properties(
        &self,
        ui: &mut egui::Ui,
        headers: &mut Vec<String>,
        rows: &mut Vec<Vec<String>>,
    ) {
        ui.label("Headers:");
        for (i, header) in headers.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("{}:", i + 1));
                ui.text_edit_singleline(header);
            });
        }

        ui.horizontal(|ui| {
            if ui.button("➕ Add Header").clicked() {
                headers.push(format!("Header {}", headers.len() + 1));
            }
            if ui.button("➖ Remove Header").clicked() && !headers.is_empty() {
                headers.pop();
                for row in rows.iter_mut() {
                    if row.len() > headers.len() {
                        row.pop();
                    }
                }
            }
        });

        ui.separator();
        ui.label(format!("Rows ({}):", rows.len()));

        if ui.button("➕ Add Row").clicked() {
            let new_row = (0..headers.len())
                .map(|i| format!("Cell {}", i + 1))
                .collect();
            rows.push(new_row);
        }

        if ui.button("➖ Remove Row").clicked() && !rows.is_empty() {
            rows.pop();
        }
    }

    fn render_text_properties(&self, ui: &mut egui::Ui, content: &mut String, font_size: &mut f32) {
        ui.label("Content:");
        ui.text_edit_multiline(content);

        ui.horizontal(|ui| {
            ui.label("Font Size:");
            ui.add(
                egui::DragValue::new(font_size)
                    .speed(0.5)
                    .clamp_range(8.0..=72.0),
            );
        });
    }

    fn render_button_properties(&self, ui: &mut egui::Ui, label: &mut String, action: &mut String) {
        ui.horizontal(|ui| {
            ui.label("Label:");
            ui.text_edit_singleline(label);
        });

        ui.horizontal(|ui| {
            ui.label("Action:");
            ui.text_edit_singleline(action);
        });
    }

    fn render_image_properties(&self, ui: &mut egui::Ui, path: &mut String, scale: &mut f32) {
        ui.horizontal(|ui| {
            ui.label("Path:");
            ui.text_edit_singleline(path);
        });

        ui.horizontal(|ui| {
            ui.label("Scale:");
            ui.add(
                egui::DragValue::new(scale)
                    .speed(0.01)
                    .clamp_range(0.1..=5.0),
            );
        });
    }
}
