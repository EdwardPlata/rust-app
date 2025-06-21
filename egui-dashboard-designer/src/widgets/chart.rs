use eframe::egui;
use egui_plot::{Bar, BarChart, Line, Plot, PlotPoints, Points};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
    pub data: Vec<(f64, f64)>,
    pub title: String,
    pub chart_type: ChartType,
    pub show_axes: bool,
    pub show_grid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChartType {
    Line,
    Scatter,
    Bar,
}

impl Chart {
    pub fn new(title: String) -> Self {
        Self {
            data: vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.5), (3.0, 2.0)],
            title,
            chart_type: ChartType::Line,
            show_axes: true,
            show_grid: true,
        }
    }

    pub fn update_data(&mut self, new_data: Vec<(f64, f64)>) {
        self.data = new_data;
    }

    pub fn set_chart_type(&mut self, chart_type: ChartType) {
        self.chart_type = chart_type;
    }

    pub fn render(&self, ui: &mut egui::Ui, size: egui::Vec2) {
        let plot = Plot::new(&self.title)
            .view_aspect(2.0)
            .show_axes(self.show_axes)
            .show_grid(self.show_grid)
            .allow_zoom(true)
            .allow_drag(true)
            .width(size.x - 20.0)
            .height(size.y - 60.0);

        plot.show(ui, |plot_ui| match self.chart_type {
            ChartType::Line => {
                let points: PlotPoints = self.data.iter().map(|(x, y)| [*x, *y]).collect();
                let line = Line::new(points)
                    .color(egui::Color32::BLUE)
                    .stroke(egui::Stroke::new(2.0, egui::Color32::BLUE));
                plot_ui.line(line);
            }
            ChartType::Scatter => {
                let points: PlotPoints = self.data.iter().map(|(x, y)| [*x, *y]).collect();
                let scatter = Points::new(points).color(egui::Color32::RED).radius(4.0);
                plot_ui.points(scatter);
            }
            ChartType::Bar => {
                let bars: Vec<Bar> = self
                    .data
                    .iter()
                    .enumerate()
                    .map(|(i, (_, y))| Bar::new(i as f64, *y).width(0.8))
                    .collect();
                let chart = BarChart::new(bars).color(egui::Color32::GREEN);
                plot_ui.bar_chart(chart);
            }
        });
    }
}
