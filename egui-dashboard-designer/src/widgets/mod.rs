mod chart;
mod gauge;
mod table;

pub use chart::{Chart, ChartType};
pub use gauge::Gauge;
pub use table::Table;

use eframe::egui;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(D            WidgetData::Text { content, font_size } => {
                child_ui.label(egui::RichText::new(content.as_str()).size(*font_size));
            },
            WidgetData::Button { label, .. } => {
                child_ui.button(label.as_str());
            },Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum WidgetType {
    Chart,
    Gauge,
    Table,
    Text,
    Button,
    Image,
}

impl WidgetType {
    pub fn all() -> Vec<WidgetType> {
        vec![
            WidgetType::Chart,
            WidgetType::Gauge,
            WidgetType::Table,
            WidgetType::Text,
            WidgetType::Button,
            WidgetType::Image,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            WidgetType::Chart => "Chart",
            WidgetType::Gauge => "Gauge",
            WidgetType::Table => "Table",
            WidgetType::Text => "Text",
            WidgetType::Button => "Button",
            WidgetType::Image => "Image",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardWidget {
    pub id: Uuid,
    pub widget_type: WidgetType,
    pub position: egui::Pos2,
    pub size: egui::Vec2,
    pub title: String,
    pub data: WidgetData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetData {
    Chart {
        data_points: Vec<(f64, f64)>,
        chart_type: ChartType,
    },
    Gauge {
        value: f64,
        min: f64,
        max: f64,
        unit: String,
    },
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    Text {
        content: String,
        font_size: f32,
    },
    Button {
        label: String,
        action: String,
    },
    Image {
        path: String,
        scale: f32,
    },
}

impl DashboardWidget {
    pub fn new(widget_type: WidgetType, position: egui::Pos2) -> Self {
        let data = match widget_type {
            WidgetType::Chart => WidgetData::Chart {
                data_points: vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.5), (3.0, 2.0)],
                chart_type: ChartType::Line,
            },
            WidgetType::Gauge => WidgetData::Gauge {
                value: 75.0,
                min: 0.0,
                max: 100.0,
                unit: "%".to_string(),
            },
            WidgetType::Table => WidgetData::Table {
                headers: vec!["Name".to_string(), "Value".to_string()],
                rows: vec![
                    vec!["Item 1".to_string(), "100".to_string()],
                    vec!["Item 2".to_string(), "200".to_string()],
                ],
            },
            WidgetType::Text => WidgetData::Text {
                content: "Sample Text".to_string(),
                font_size: 14.0,
            },
            WidgetType::Button => WidgetData::Button {
                label: "Click Me".to_string(),
                action: "action".to_string(),
            },
            WidgetType::Image => WidgetData::Image {
                path: "path/to/image.png".to_string(),
                scale: 1.0,
            },
        };

        Self {
            id: Uuid::new_v4(),
            widget_type,
            position,
            size: egui::vec2(200.0, 150.0),
            title: format!("{} Widget", widget_type.name()),
            data,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, is_selected: bool) -> egui::Response {
        let rect = egui::Rect::from_min_size(self.position, self.size);

        let response = ui.allocate_rect(rect, egui::Sense::click_and_drag());

        if response.dragged() {
            self.position += response.drag_delta();
        }

        let visuals = if is_selected {
            ui.visuals().widgets.active
        } else if response.hovered() {
            ui.visuals().widgets.hovered
        } else {
            ui.visuals().widgets.inactive
        };

        ui.painter().rect(
            rect,
            egui::Rounding::same(4.0),
            visuals.bg_fill,
            egui::Stroke::new(2.0, visuals.bg_stroke.color),
        );

        // Render widget content
        let inner_rect = rect.shrink(8.0);
        let mut child_ui = ui.child_ui(inner_rect, egui::Layout::top_down(egui::Align::LEFT), None);

        child_ui.label(&self.title);
        child_ui.separator();

        match &mut self.data {
            WidgetData::Chart { data_points, .. } => {
                self.render_chart(&mut child_ui, data_points);
            }
            WidgetData::Gauge {
                value,
                min,
                max,
                unit,
            } => {
                self.render_gauge(&mut child_ui, *value, *min, *max, unit);
            }
            WidgetData::Table { headers, rows } => {
                self.render_table(&mut child_ui, headers, rows);
            }
            WidgetData::Text { content, font_size } => {
                child_ui.label(egui::RichText::new(content).size(*font_size));
            }
            WidgetData::Button { label, .. } => {
                child_ui.button(label);
            }
            WidgetData::Image { path, .. } => {
                child_ui.label(format!("Image: {}", path));
            }
        }

        response
    }

    fn render_chart(&self, ui: &mut egui::Ui, data_points: &[(f64, f64)]) {
        if let Some(chart) = self.get_chart_instance(data_points) {
            chart.render(ui, self.size);
        } else {
            ui.label("📊 Chart Data");
            for (i, (x, y)) in data_points.iter().take(3).enumerate() {
                ui.label(format!("Point {}: ({:.1}, {:.1})", i + 1, x, y));
            }
            if data_points.len() > 3 {
                ui.label(format!("... and {} more points", data_points.len() - 3));
            }
        }
    }

    fn render_gauge(&self, ui: &mut egui::Ui, value: f64, min: f64, max: f64, unit: &str) {
        if let Some(gauge) = self.get_gauge_instance(value, min, max, unit) {
            gauge.render(ui, self.size);
        } else {
            ui.label("📊 Gauge");
            ui.label(format!("Value: {:.1}{}", value, unit));
            ui.label(format!("Range: {:.1} - {:.1}{}", min, max, unit));

            let progress = (value - min) / (max - min);
            ui.add(
                egui::ProgressBar::new(progress as f32).text(format!("{:.1}%", progress * 100.0)),
            );
        }
    }

    fn render_table(&self, ui: &mut egui::Ui, headers: &[String], rows: &[Vec<String>]) {
        if let Some(mut table) = self.get_table_instance(headers, rows) {
            table.render(ui, self.size);
        } else {
            ui.label("📋 Table");
            ui.label(format!("Headers: {}", headers.join(", ")));
            ui.label(format!("Rows: {}", rows.len()));
        }
    }

    // Helper methods to create widget instances
    fn get_chart_instance(&self, data_points: &[(f64, f64)]) -> Option<Chart> {
        if let WidgetData::Chart { chart_type, .. } = &self.data {
            let mut chart = Chart::new(self.title.clone());
            chart.update_data(data_points.to_vec());
            chart.set_chart_type(chart_type.clone());
            Some(chart)
        } else {
            None
        }
    }

    fn get_gauge_instance(&self, value: f64, min: f64, max: f64, unit: &str) -> Option<Gauge> {
        let mut gauge = Gauge::new(self.title.clone(), min, max);
        gauge.set_value(value);
        gauge.unit = unit.to_string();
        Some(gauge)
    }

    fn get_table_instance(&self, headers: &[String], rows: &[Vec<String>]) -> Option<Table> {
        let table = Table::new(self.title.clone(), headers.to_vec(), rows.to_vec());
        Some(table)
    }
}
