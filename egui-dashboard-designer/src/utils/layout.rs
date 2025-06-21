use eframe::egui;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GridLayout {
    pub rows: usize,
    pub columns: usize,
    pub cell_size: egui::Vec2,
    pub spacing: f32,
}

impl Default for GridLayout {
    fn default() -> Self {
        Self {
            rows: 4,
            columns: 6,
            cell_size: egui::vec2(200.0, 150.0),
            spacing: 10.0,
        }
    }
}

impl GridLayout {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            rows,
            columns,
            ..Default::default()
        }
    }

    pub fn get_cell_position(&self, row: usize, col: usize) -> egui::Pos2 {
        egui::pos2(
            col as f32 * (self.cell_size.x + self.spacing) + self.spacing,
            row as f32 * (self.cell_size.y + self.spacing) + self.spacing,
        )
    }

    pub fn get_grid_cell(&self, position: egui::Pos2) -> (usize, usize) {
        let col =
            ((position.x - self.spacing) / (self.cell_size.x + self.spacing)).floor() as usize;
        let row =
            ((position.y - self.spacing) / (self.cell_size.y + self.spacing)).floor() as usize;
        (row.min(self.rows - 1), col.min(self.columns - 1))
    }

    pub fn snap_to_grid(&self, position: egui::Pos2) -> egui::Pos2 {
        let (row, col) = self.get_grid_cell(position);
        self.get_cell_position(row, col)
    }
}

#[derive(Debug, Clone)]
pub enum LayoutMode {
    FreeForm,
    Grid(GridLayout),
    Auto,
}

impl Default for LayoutMode {
    fn default() -> Self {
        LayoutMode::FreeForm
    }
}

pub fn auto_arrange_widgets(
    widget_positions: &mut Vec<(Uuid, egui::Pos2, egui::Vec2)>,
    container_size: egui::Vec2,
) {
    if widget_positions.is_empty() {
        return;
    }

    // Simple auto-arrangement: arrange widgets in a grid pattern
    let widget_count = widget_positions.len();
    let cols = (widget_count as f32).sqrt().ceil() as usize;
    let rows = (widget_count + cols - 1) / cols;

    let cell_width = container_size.x / cols as f32;
    let cell_height = container_size.y / rows as f32;

    for (index, (_, position, size)) in widget_positions.iter_mut().enumerate() {
        let row = index / cols;
        let col = index % cols;

        position.x = col as f32 * cell_width + 10.0;
        position.y = row as f32 * cell_height + 10.0;

        // Adjust size to fit cell with some padding
        size.x = (cell_width - 20.0).max(100.0);
        size.y = (cell_height - 20.0).max(80.0);
    }
}

pub fn detect_collisions(widgets: &[(Uuid, egui::Pos2, egui::Vec2)]) -> Vec<(Uuid, Uuid)> {
    let mut collisions = Vec::new();

    for (i, (id1, pos1, size1)) in widgets.iter().enumerate() {
        for (id2, pos2, size2) in widgets.iter().skip(i + 1) {
            let rect1 = egui::Rect::from_min_size(*pos1, *size1);
            let rect2 = egui::Rect::from_min_size(*pos2, *size2);

            if rect1.intersects(rect2) {
                collisions.push((*id1, *id2));
            }
        }
    }

    collisions
}

pub fn align_widgets(
    widget_positions: &mut [(Uuid, egui::Pos2, egui::Vec2)],
    alignment: Alignment,
) {
    if widget_positions.is_empty() {
        return;
    }

    match alignment {
        Alignment::Left => {
            let min_x = widget_positions
                .iter()
                .map(|(_, pos, _)| pos.x)
                .fold(f32::INFINITY, f32::min);
            for (_, pos, _) in widget_positions.iter_mut() {
                pos.x = min_x;
            }
        }
        Alignment::Right => {
            let max_x = widget_positions
                .iter()
                .map(|(_, pos, size)| pos.x + size.x)
                .fold(f32::NEG_INFINITY, f32::max);
            for (_, pos, size) in widget_positions.iter_mut() {
                pos.x = max_x - size.x;
            }
        }
        Alignment::Top => {
            let min_y = widget_positions
                .iter()
                .map(|(_, pos, _)| pos.y)
                .fold(f32::INFINITY, f32::min);
            for (_, pos, _) in widget_positions.iter_mut() {
                pos.y = min_y;
            }
        }
        Alignment::Bottom => {
            let max_y = widget_positions
                .iter()
                .map(|(_, pos, size)| pos.y + size.y)
                .fold(f32::NEG_INFINITY, f32::max);
            for (_, pos, size) in widget_positions.iter_mut() {
                pos.y = max_y - size.y;
            }
        }
        Alignment::CenterHorizontal => {
            let center_x = widget_positions
                .iter()
                .map(|(_, pos, size)| pos.x + size.x / 2.0)
                .sum::<f32>()
                / widget_positions.len() as f32;
            for (_, pos, size) in widget_positions.iter_mut() {
                pos.x = center_x - size.x / 2.0;
            }
        }
        Alignment::CenterVertical => {
            let center_y = widget_positions
                .iter()
                .map(|(_, pos, size)| pos.y + size.y / 2.0)
                .sum::<f32>()
                / widget_positions.len() as f32;
            for (_, pos, size) in widget_positions.iter_mut() {
                pos.y = center_y - size.y / 2.0;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Left,
    Right,
    Top,
    Bottom,
    CenterHorizontal,
    CenterVertical,
}
