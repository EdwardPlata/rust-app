use eframe::egui;
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gauge {
    pub value: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub unit: String,
    pub title: String,
    pub show_value: bool,
    pub color_zones: Vec<ColorZone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorZone {
    pub min: f64,
    pub max: f64,
    pub color: [u8; 3], // RGB values
}

impl Gauge {
    pub fn new(title: String, min_value: f64, max_value: f64) -> Self {
        Self {
            value: (min_value + max_value) / 2.0,
            min_value,
            max_value,
            unit: String::new(),
            title,
            show_value: true,
            color_zones: vec![
                ColorZone {
                    min: min_value,
                    max: max_value * 0.7,
                    color: [0, 255, 0],
                }, // Green
                ColorZone {
                    min: max_value * 0.7,
                    max: max_value * 0.9,
                    color: [255, 255, 0],
                }, // Yellow
                ColorZone {
                    min: max_value * 0.9,
                    max: max_value,
                    color: [255, 0, 0],
                }, // Red
            ],
        }
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value.clamp(self.min_value, self.max_value);
    }

    pub fn get_value_color(&self) -> egui::Color32 {
        for zone in &self.color_zones {
            if self.value >= zone.min && self.value <= zone.max {
                return egui::Color32::from_rgb(zone.color[0], zone.color[1], zone.color[2]);
            }
        }
        egui::Color32::GRAY
    }

    pub fn render(&self, ui: &mut egui::Ui, size: egui::Vec2) {
        let center = egui::pos2(size.x / 2.0, size.y / 2.0);
        let radius = (size.x.min(size.y) / 2.0 - 20.0).max(30.0);

        let painter = ui.painter();

        // Draw gauge background arc
        let start_angle = PI * 1.25; // Start from bottom-left
        let end_angle = PI * 1.75; // End at bottom-right
        let arc_length = end_angle - start_angle;

        // Draw background arc
        self.draw_arc(
            painter,
            center,
            radius,
            start_angle,
            arc_length,
            8.0,
            egui::Color32::GRAY,
        );

        // Draw value arc
        let percentage = (self.value - self.min_value) / (self.max_value - self.min_value);
        let value_arc_length = arc_length * percentage as f32;
        let value_color = self.get_value_color();

        self.draw_arc(
            painter,
            center,
            radius,
            start_angle,
            value_arc_length,
            8.0,
            value_color,
        );

        // Draw needle
        let needle_angle = start_angle + value_arc_length;
        let needle_end = egui::pos2(
            center.x + radius * needle_angle.cos(),
            center.y + radius * needle_angle.sin(),
        );

        painter.line_segment(
            [center, needle_end],
            egui::Stroke::new(3.0, egui::Color32::BLACK),
        );

        // Draw center circle
        painter.circle_filled(center, 5.0, egui::Color32::BLACK);

        // Draw value text
        if self.show_value {
            let value_text = format!("{:.1}{}", self.value, self.unit);
            let text_pos = egui::pos2(center.x, center.y + radius + 15.0);
            painter.text(
                text_pos,
                egui::Align2::CENTER_CENTER,
                &value_text,
                egui::FontId::proportional(16.0),
                egui::Color32::BLACK,
            );
        }

        // Draw min/max labels
        let min_pos = egui::pos2(
            center.x + radius * start_angle.cos() * 0.8,
            center.y + radius * start_angle.sin() * 0.8,
        );
        let max_pos = egui::pos2(
            center.x + radius * end_angle.cos() * 0.8,
            center.y + radius * end_angle.sin() * 0.8,
        );

        painter.text(
            min_pos,
            egui::Align2::CENTER_CENTER,
            &format!("{:.0}", self.min_value),
            egui::FontId::proportional(12.0),
            egui::Color32::DARK_GRAY,
        );

        painter.text(
            max_pos,
            egui::Align2::CENTER_CENTER,
            &format!("{:.0}", self.max_value),
            egui::FontId::proportional(12.0),
            egui::Color32::DARK_GRAY,
        );
    }

    fn draw_arc(
        &self,
        painter: &egui::Painter,
        center: egui::Pos2,
        radius: f32,
        start_angle: f32,
        arc_length: f32,
        thickness: f32,
        color: egui::Color32,
    ) {
        let segments = ((arc_length * radius / 5.0).ceil() as usize).max(8);
        let angle_step = arc_length / segments as f32;

        for i in 0..segments {
            let angle1 = start_angle + i as f32 * angle_step;
            let angle2 = start_angle + (i + 1) as f32 * angle_step;

            let p1 = egui::pos2(
                center.x + (radius - thickness / 2.0) * angle1.cos(),
                center.y + (radius - thickness / 2.0) * angle1.sin(),
            );
            let p2 = egui::pos2(
                center.x + (radius - thickness / 2.0) * angle2.cos(),
                center.y + (radius - thickness / 2.0) * angle2.sin(),
            );

            painter.line_segment([p1, p2], egui::Stroke::new(thickness, color));
        }
    }
}
