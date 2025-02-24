use super::Homepage;
use crate::{
    app::{IPage, PageEvent},
    config::Config,
    widgets::Button,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    text::Line,
    Frame,
};

#[derive(Debug, Clone, Default)]
pub struct ConfigPage {
    current_field: ConfigField,
}

impl IPage for ConfigPage {
    fn render(&mut self, frame: &mut Frame, main_area: Rect, _: &Config) {
        let [title_area, _, bottom_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(main_area);

        let page_title = Line::from("\u{1F527} Configuration").bold().centered();
        frame.render_widget(page_title, title_area);

        let [left_button, _, right_button] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .flex(Flex::Center)
        .areas(bottom_area);
        frame.render_widget(
            Button::new("Cancel", self.current_field == ConfigField::CancelButton).right_aligned(),
            left_button,
        );
        frame.render_widget(
            Button::new("Save", self.current_field == ConfigField::SaveButton),
            right_button,
        );
    }

    fn handle_key_events(&mut self, key_event: KeyEvent, _: &mut Config) -> PageEvent {
        if key_event.code == KeyCode::Esc {
            return PageEvent::Navigate(Homepage::default().into());
        }

        match (&self.current_field, key_event.code) {
            (ConfigField::CancelButton, KeyCode::Enter | KeyCode::Char(' ')) => {
                return PageEvent::Navigate(Homepage::default().into());
            }
            (ConfigField::SaveButton, KeyCode::Enter | KeyCode::Char(' ')) => {
                // TODO should save config
                return PageEvent::Navigate(Homepage::default().into());
            }
            (_, KeyCode::Left | KeyCode::Up | KeyCode::Tab) => match self.current_field {
                ConfigField::CancelButton => self.current_field = ConfigField::SaveButton,
                ConfigField::SaveButton => self.current_field = ConfigField::CancelButton,
            },
            (_, KeyCode::Right | KeyCode::Down) => match self.current_field {
                ConfigField::CancelButton => self.current_field = ConfigField::SaveButton,
                ConfigField::SaveButton => self.current_field = ConfigField::CancelButton,
            },
            _ => {}
        }

        PageEvent::Nothing
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
enum ConfigField {
    CancelButton,
    #[default]
    SaveButton,
}
