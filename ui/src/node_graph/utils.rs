use eframe::epaint::CubicBezierShape;
use egui::{Color32, Painter, Pos2, Stroke, Vec2};
use std::f32::consts as f32_consts;

pub fn draw_connection(painter: &Painter, start: Pos2, end: Pos2, stroke: Stroke) {
    let distance = start.distance(end);
    let half_distance = distance * 0.5;
    if distance < 5. {
        return;
    }

    fn easy_strength(half_distance: f32, strength: f32) -> f32 {
        if half_distance < strength {
            strength * (f32_consts::PI * 0.5 * half_distance / strength).sin()
        } else {
            strength
        }
    }

    let start_strength = easy_strength(half_distance, 100.);
    let cp0 = start - Vec2::X * start_strength;
    let end_strength = easy_strength(half_distance, 300.);
    let cp1 = end + Vec2::X * end_strength;

    let bezier = CubicBezierShape::from_points_stroke([start, cp0, cp1, end], false, Color32::TRANSPARENT, stroke);

    painter.add(bezier);
}
