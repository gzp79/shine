use crate::utils::Scale;
use egui::{pos2, vec2, Id, Pos2, Rect, Style, Ui, Vec2};
use shine_core::collections::HashWithType;
use std::{any::Any, hash::Hash, sync::Arc};

#[derive(Clone)]
pub(in crate::node_graph) struct ZoomPanState {
    pub id: Id,
    pub pan: Vec2,
    pub zoom: f32,
    pub screen_rect: Rect,

    pub default_style: Arc<Style>,
    pub zoomed_style: Arc<Style>,
}

impl ZoomPanState {
    pub fn load(ui: &mut Ui, id: Id) -> Option<ZoomPanState> {
        ui.data().get_temp(id)
    }

    pub fn store(self, ui: &mut Ui, id: Id) {
        ui.data().insert_temp(id, self);
    }

    pub fn new(id: Id, ui: &mut Ui) -> Self {
        Self {
            id,
            pan: Vec2::ZERO,
            zoom: 1.,
            screen_rect: Rect::NOTHING,
            default_style: ui.style().clone(),
            zoomed_style: ui.style().clone(),
        }
    }

    pub fn child_id<I: Any + Hash>(&self, id: I) -> Id {
        self.id.with(HashWithType(id))
    }

    /*pub fn vec2_area_to_screen(&self, v: Vec2) -> Vec2 {
        vec2(v.x * self.zoom, v.y * self.zoom)
    }*/

    pub fn pos2_area_to_screen(&self, p: Pos2) -> Pos2 {
        let Pos2 { x, y } = p;
        let x = x + self.screen_rect.left();
        let y = y + self.screen_rect.top();
        let x = (x + self.pan.x) * self.zoom;
        let y = (y + self.pan.y) * self.zoom;
        pos2(x, y)
    }

    pub fn vec2_screen_to_area(&self, v: Vec2) -> Vec2 {
        vec2(v.x / self.zoom, v.y / self.zoom)
    }

    pub fn pos2_screen_to_area(&self, p: Pos2) -> Pos2 {
        let Pos2 { x, y } = p;
        let x = x / self.zoom - self.pan.x;
        let y = y / self.zoom - self.pan.y;
        let x = x - self.screen_rect.left();
        let y = y - self.screen_rect.top();
        pos2(x, y)
    }

    pub fn drag(&mut self, delta: Vec2) {
        let delta = self.vec2_screen_to_area(delta);
        self.update(self.pan + delta, self.zoom);
    }

    pub fn zoom_to_screen(&mut self, screen_pos: Pos2, zoom: f32) {
        let new_zoom = (self.zoom * zoom).clamp(0.1, 10.);

        // keep the screen_pos remain at the same location
        // solved for the equations: a2s_pos_zoom(s2a_pre_zoom(screen_pos)) = screen_pos

        let test = self.pos2_screen_to_area(screen_pos);

        let Pos2 { x, y } = screen_pos;
        let new_pan = vec2(
            x / new_zoom - x / self.zoom + self.pan.x,
            y / new_zoom - y / self.zoom + self.pan.y,
        );

        let err = self.pos2_area_to_screen(test) - screen_pos;
        assert!(err.x < 1.);
        assert!(err.y < 1.);
        self.update(new_pan, new_zoom);
    }

    pub fn prepare(&mut self, style: &Arc<Style>) {
        self.default_style = style.clone();
    }

    pub fn update(&mut self, pan: Vec2, zoom: f32) {
        if self.zoom != zoom {
            self.zoomed_style = Arc::new(self.default_style.scaled(self.zoom));
        }
        self.pan = pan;
        self.zoom = zoom;
    }

    pub fn show_zoomed<R, F>(&self, ui: &mut Ui, add_content: F) -> R
    where
        F: FnOnce(&mut Ui) -> R,
    {
        let original_cliprect = ui.clip_rect();
        ui.set_clip_rect(self.screen_rect);
        ui.ctx().set_style(self.zoomed_style.clone());
        let response = add_content(ui);
        ui.ctx().set_style(self.default_style.clone());
        ui.set_clip_rect(original_cliprect);

        response
    }

    pub fn show_clipped<R, F>(&self, ui: &mut Ui, add_content: F) -> R
    where
        F: FnOnce(&mut Ui) -> R,
    {
        let original_cliprect = ui.clip_rect();
        ui.set_clip_rect(self.screen_rect);
        let response = add_content(ui);
        ui.set_clip_rect(original_cliprect);

        response
    }
}
