use egui::{pos2, Frame, Rect, WidgetText, Ui, TextStyle, Vec2};

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
            let top = ui.min_rect().bottom();

            let title_galley = self.title.into_galley(ui, Some(false), f32::INFINITY, TextStyle::Heading);

            let (height, top_margin) = {
                let style = ui.style();
                let h = title_galley.size().y;
                //let h = self.title.font_height(&ui.ctx().fonts(), style);
                let top_margin = style.spacing.item_spacing.y * 2.;
                (h + top_margin, top_margin)
            };
            ui.add_space(height + top_margin);

            add_content(ui);

            let frame_rect = ui.min_rect();
            title_rect = Rect::from_min_max(pos2(frame_rect.left(), top), pos2(frame_rect.right(), top + height));            
            let text_pos = emath::align::center_size_in_rect(title_galley.size(), title_rect).left_top();
            let text_pos = text_pos - title_galley.galley().rect.min.to_vec2();
            let text_pos = text_pos - top_margin * Vec2::Y; // HACK: center on x-height of text (looks better)
            title_galley.paint_with_fallback_color(
                ui.painter(),
                text_pos,
                ui.visuals().text_color(),
            );

        });

        // HACK: moved sperartor here to make it aligned to the frame horizontally.
        let stroke = ui.visuals().widgets.noninteractive.bg_stroke;
        ui.painter().hline(ui.min_rect().x_range(), title_rect.bottom(), stroke);
    }
}
