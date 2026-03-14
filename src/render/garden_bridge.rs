use crate::geometry::{Point, Bounds};
use eframe::egui;

/// Convert internal Point to egui::Pos2 for rendering
pub fn point_to_egui(p: Point) -> egui::Pos2 {
    egui::Pos2 { x: p.x, y: p.y }
}

/// Convert internal Bounds to egui::Rect for rendering
pub fn bounds_to_egui(b: Bounds) -> egui::Rect {
    egui::Rect::from_min_max(
        egui::Pos2 {
            x: b.min_x,
            y: b.min_y,
        },
        egui::Pos2 {
            x: b.max_x,
            y: b.max_y,
        },
    )
}

/// Convert egui::Pos2 to internal Point
pub fn egui_pos_to_point(p: egui::Pos2) -> Point {
    Point::new(p.x, p.y)
}

/// Convert egui::Rect to internal Bounds
pub fn egui_rect_to_bounds(r: egui::Rect) -> Bounds {
    Bounds::new(r.min.x, r.min.y, r.max.x, r.max.y)
}
