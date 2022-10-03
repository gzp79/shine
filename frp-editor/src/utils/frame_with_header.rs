use egui::{pos2, Align, Frame, Label, Layout, Rect, RichText, Separator, Ui};

/// Draw a header by reserving it fist and
pub struct FrameWithHeader {
    title: RichText,
    frame: Option<Frame>,
}

impl FrameWithHeader {
    pub fn new<S: Into<RichText>>(text: S) -> Self {
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
        self.frame.unwrap_or_default().show(ui, |ui| {
            let top = ui.min_rect().bottom();
            let height = {
                let style = ui.style();
                self.title.font_height(&ui.ctx().fonts(), style) + style.spacing.item_spacing.y * 3.0
            };
            ui.add_space(height);

            add_content(ui);

            let frame_rect = ui.min_rect();
            let title_rect = Rect::from_min_max(pos2(frame_rect.left(), top), pos2(frame_rect.right(), top + height));
            let mut child_ui = ui.child_ui(title_rect, Layout::top_down_justified(Align::Center));
            child_ui.add(Label::new(self.title));
            child_ui.add(Separator::default().spacing(0.));
        });
    }
}
