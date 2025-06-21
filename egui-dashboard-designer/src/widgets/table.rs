use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub data: Vec<Vec<String>>,
    pub headers: Vec<String>,
    pub title: String,
    pub sortable: bool,
    pub filterable: bool,
    pub striped: bool,
    pub sort_column: Option<usize>,
    pub sort_ascending: bool,
}

impl Table {
    pub fn new(title: String, headers: Vec<String>, data: Vec<Vec<String>>) -> Self {
        Self {
            headers,
            data,
            title,
            sortable: true,
            filterable: false,
            striped: true,
            sort_column: None,
            sort_ascending: true,
        }
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        // Ensure row has the correct number of columns
        let mut padded_row = row;
        padded_row.resize(self.headers.len(), String::new());
        self.data.push(padded_row);
    }

    pub fn add_column(&mut self, header: String, default_value: String) {
        self.headers.push(header);
        for row in &mut self.data {
            row.push(default_value.clone());
        }
    }

    pub fn sort_by_column(&mut self, column_index: usize) {
        if column_index >= self.headers.len() {
            return;
        }

        let ascending = if Some(column_index) == self.sort_column {
            !self.sort_ascending
        } else {
            true
        };

        self.sort_column = Some(column_index);
        self.sort_ascending = ascending;

        self.data.sort_by(|a, b| {
            let empty_string = String::new();
            let cell_a = a.get(column_index).unwrap_or(&empty_string);
            let cell_b = b.get(column_index).unwrap_or(&empty_string);

            // Try to parse as numbers first
            if let (Ok(num_a), Ok(num_b)) = (cell_a.parse::<f64>(), cell_b.parse::<f64>()) {
                if ascending {
                    num_a.partial_cmp(&num_b).unwrap_or(std::cmp::Ordering::Equal)
                } else {
                    num_b.partial_cmp(&num_a).unwrap_or(std::cmp::Ordering::Equal)
                }
            } else {
                // Fall back to string comparison
                if ascending {
                    cell_a.cmp(cell_b)
                } else {
                    cell_b.cmp(cell_a)
                }
            }
        });
    }

    pub fn render(&mut self, ui: &mut egui::Ui, size: egui::Vec2) {
        if self.headers.is_empty() {
            ui.label("No data to display");
            return;
        }

        // Simple table rendering without complex sorting UI for now
        ui.group(|ui| {
            // Headers
            ui.horizontal(|ui| {
                for (i, header) in self.headers.iter().enumerate() {
                    let mut header_text = header.clone();
                    if Some(i) == self.sort_column {
                        header_text.push_str(if self.sort_ascending { " ↑" } else { " ↓" });
                    }
                    
                    if ui.button(&header_text).clicked() && self.sortable {
                        self.sort_by_column(i);
                    }
                }
            });
            
            ui.separator();
            
            // Data rows
            egui::ScrollArea::vertical()
                .max_height(size.y - 80.0)
                .show(ui, |ui| {
                    for row in &self.data {
                        ui.horizontal(|ui| {
                            for cell in row {
                                ui.label(cell);
                            }
                        });
                    }
                });
        });

        // Show row count
        ui.horizontal(|ui| {
            ui.label(format!("Rows: {}", self.data.len()));
            ui.label(format!("Columns: {}", self.headers.len()));
        });
    }
}