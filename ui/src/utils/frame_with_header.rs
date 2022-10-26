use egui::{pos2, vec2, Frame, Rect, TextStyle, Ui, Vec2, WidgetText};

/// Draw a header by reserving it fist and
pub struct FrameWithHeader {
    title: WidgetText,
    frame: Option<Frame>,
}

impl FrameWithHeader {
    pub fn new<S: Into<WidgetText>>(text: S) -> Self {
        let title = text.into().heading();
        Self { title, frame: None }
    }

    pub fn frame(self, frame: Frame) -> Self {
        Self {
            frame: Some(frame),
            ..self
        }
    }

    pub fn show<F>(self, ui: &mut Ui, add_content: F)
    where
        F: FnOnce(&mut Ui),
    {
        let mut title_rect = Rect::NOTHING;
        self.frame.unwrap_or_default().show(ui, |ui| {
            let top_left = ui.min_rect().min;

            let title_galley = self
                .title
                .into_galley(ui, Some(false), f32::INFINITY, TextStyle::Heading);

            let (tile_size, top_margin) = {
                let style = ui.style();
                let size = title_galley.size();
                let w = size.x;
                let h = size.y;
                //let h = self.title.font_height(&ui.ctx().fonts(), style);
                let top_margin = style.spacing.item_spacing.y * 2.;
                (vec2(w, h + top_margin), top_margin)
            };
            ui.add_space(tile_size.y + top_margin);

            add_content(ui);

            // make sure header is enclosed in the frame
            let frame_rect = ui.min_rect();
            let header_title_rect = Rect::from_min_size(top_left, tile_size);
            let content_title_rect = Rect::from_min_max(
                pos2(frame_rect.left(), top_left.y),
                pos2(frame_rect.right(), top_left.y + tile_size.y),
            );
            title_rect = header_title_rect.union(content_title_rect);
            ui.expand_to_include_rect(title_rect);

            // align header to center
            let text_pos = emath::align::center_size_in_rect(title_galley.size(), title_rect).left_top();
            let text_pos = text_pos - title_galley.galley().rect.min.to_vec2();
            let text_pos = text_pos - top_margin * Vec2::Y; // HACK: center on x-height of text (looks better)

            title_galley.paint_with_fallback_color(ui.painter(), text_pos, ui.visuals().text_color());
        });

        // HACK: moved sperartor here to make it aligned to the frame horizontally.
        let stroke = ui.visuals().widgets.noninteractive.bg_stroke;
        ui.painter().hline(ui.min_rect().x_range(), title_rect.bottom(), stroke);
    }
}
