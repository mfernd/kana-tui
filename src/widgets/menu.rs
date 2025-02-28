use ratatui::{
    layout::Alignment,
    style::Stylize,
    text::Line,
    widgets::{Paragraph, StatefulWidget, Widget},
};

#[derive(Debug)]
pub struct Menu<'a> {
    options: Vec<&'a str>,
    alignment: Alignment,
    spacing: usize,
}

#[allow(dead_code)]
impl<'a> Menu<'a> {
    pub fn new(options: Vec<&'a str>) -> Self {
        Self {
            options,
            alignment: Alignment::default(),
            spacing: 1,
        }
    }

    pub fn centered(mut self) -> Self {
        self.alignment = Alignment::Center;
        self
    }

    pub fn with_spacing(mut self, spacing: usize) -> Self {
        self.spacing = spacing;
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct MenuState {
    pub current_option: usize,
}

impl MenuState {
    pub fn next_option(&mut self, max: usize) {
        self.current_option = (self.current_option + 1) % (max + 1);
    }

    pub fn previous_option(&mut self, max: usize) {
        if self.current_option == 0 {
            self.current_option = max;
        } else {
            self.current_option -= 1;
        }
    }
}

impl StatefulWidget for Menu<'_> {
    type State = MenuState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let mut lines = Vec::new();

        for (i, option) in self.options.iter().enumerate() {
            lines.push(if i == state.current_option {
                Line::from(*option).bold().underlined()
            } else {
                Line::from(*option)
            });

            // line spacing, later: with a parameter?
            if i < self.options.len() - 1 {
                for _ in 0..self.spacing {
                    lines.push(Line::from(""));
                }
            }
        }

        Paragraph::new(lines)
            .alignment(self.alignment)
            .render(area, buf);
    }
}
