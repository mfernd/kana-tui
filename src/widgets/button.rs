use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    text::Span,
    widgets::Widget,
};

#[derive(Debug)]
pub struct Button<'a> {
    label: &'a str,
    is_hover: bool,
    fg: Color,
    bg: Color,
    alignment: Alignment,
}

#[allow(dead_code)]
impl<'a> Button<'a> {
    pub fn new(label: &'a str, is_hover: bool) -> Self {
        Self {
            label,
            is_hover,
            fg: if is_hover { Color::Black } else { Color::White },
            bg: if is_hover {
                Color::LightRed
            } else {
                Color::DarkGray
            },
            alignment: Alignment::Left,
        }
    }

    pub fn centered(mut self) -> Self {
        self.alignment = Alignment::Center;
        self
    }

    pub fn right_aligned(mut self) -> Self {
        self.alignment = Alignment::Right;
        self
    }
}

impl Widget for Button<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut btn_label = format!(" {} ", self.label);
        if self.is_hover {
            btn_label = format!(" {} ", self.label);
        }

        let span = Span::from(btn_label).fg(self.fg).bg(self.bg).bold();
        let btn = match self.alignment {
            Alignment::Left => span.into_left_aligned_line(),
            Alignment::Center => span.into_centered_line(),
            Alignment::Right => span.into_right_aligned_line(),
        };
        btn.render(area, buf);
    }
}
