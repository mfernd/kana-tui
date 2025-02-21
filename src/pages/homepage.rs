use crate::{
    app::{IPage, PageEvent},
    tui,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::Stylize,
    text::{Line, ToLine},
    widgets::{Paragraph, Wrap},
    Frame,
};
use std::fmt::Display;
use strum::{EnumCount, EnumIter, IntoEnumIterator};

use super::{ConfigPage, StudyPage};

#[derive(Debug, Clone, Default)]
pub struct Homepage {
    current_option: MenuOption,
}

impl IPage for Homepage {
    fn render(&mut self, frame: &mut Frame, main_area: Rect) {
        let [slogan_area, menu_area] = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)])
            .areas(main_area.inner(Margin::new(3, 0)));

        let slogan = Paragraph::new(Vec::from(["Learn your kanas from the terminal. \u{1F5FF}"
            .to_line()
            .gray()
            .italic()
            .centered()]))
        .wrap(Wrap { trim: true });
        frame.render_widget(
            slogan,
            tui::flex(
                slogan_area,
                (Flex::Center, Constraint::Fill(1)),
                (Flex::Center, Constraint::Length(2)),
            ),
        );

        let menu = MenuOption::render(&self.current_option).centered();
        frame.render_widget(menu, menu_area);
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) -> PageEvent {
        match (&self.current_option, key_event.code) {
            (_, KeyCode::Esc | KeyCode::Char('q')) => return PageEvent::QuitApp,
            (MenuOption::Quit, KeyCode::Enter) => return PageEvent::QuitApp,
            (MenuOption::Study, KeyCode::Enter) => {
                return PageEvent::Navigate(StudyPage::default().into())
            }
            (MenuOption::Configure, KeyCode::Enter) => {
                return PageEvent::Navigate(ConfigPage {}.into())
            }
            (_, KeyCode::Right | KeyCode::Down) => self.next_option(),
            (_, KeyCode::Left | KeyCode::Up) => self.previous_option(),
            _ => {}
        }

        PageEvent::Nothing
    }
}

impl Homepage {
    fn next_option(&mut self) {
        match self.current_option {
            MenuOption::Study => self.current_option = MenuOption::Configure,
            MenuOption::Configure => self.current_option = MenuOption::Quit,
            MenuOption::Quit => self.current_option = MenuOption::Study,
        }
    }

    fn previous_option(&mut self) {
        match self.current_option {
            MenuOption::Study => self.current_option = MenuOption::Quit,
            MenuOption::Configure => self.current_option = MenuOption::Study,
            MenuOption::Quit => self.current_option = MenuOption::Configure,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, EnumIter, EnumCount)]
enum MenuOption {
    #[default]
    Study,
    Configure,
    Quit,
}

impl MenuOption {
    fn render(current_option: &MenuOption) -> Paragraph {
        let mut options: Vec<Line> = Vec::new();
        for (i, option) in MenuOption::iter().enumerate() {
            let l = if option.eq(current_option) {
                Line::from(option.to_string()).bold().underlined()
            } else {
                Line::from(option.to_string())
            };
            options.push(l);
            if i < MenuOption::COUNT - 1 {
                options.push(Line::from(""));
            }
        }
        Paragraph::new(options)
    }
}

impl Display for MenuOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Study => write!(f, "Study"),
            Self::Configure => write!(f, "Configure"),
            Self::Quit => write!(f, "Exit"),
        }
    }
}
