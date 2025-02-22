use super::{ConfigPage, StudyPage};
use crate::{
    app::{IPage, PageEvent},
    tui,
    widgets::{Menu, MenuState},
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::Stylize,
    text::ToLine,
    widgets::{Paragraph, Wrap},
    Frame,
};
use strum::{EnumCount, IntoStaticStr, VariantArray};

#[derive(Debug, Clone, Default)]
pub struct Homepage {
    menu_state: MenuState,
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

        let menu = Menu::new(MenuOption::to_vec_str()).centered();
        frame.render_stateful_widget(menu, menu_area, &mut self.menu_state);
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) -> PageEvent {
        let menu_option = &MenuOption::VARIANTS[self.menu_state.current_option];
        match (menu_option, key_event.code) {
            (_, KeyCode::Esc | KeyCode::Char('q')) => return PageEvent::QuitApp,
            (MenuOption::Quit, KeyCode::Enter | KeyCode::Char(' ')) => return PageEvent::QuitApp,
            (MenuOption::Study, KeyCode::Enter | KeyCode::Char(' ')) => {
                return PageEvent::Navigate(StudyPage::default().into());
            }
            (MenuOption::Configure, KeyCode::Enter | KeyCode::Char(' ')) => {
                return PageEvent::Navigate(ConfigPage::default().into());
            }
            (_, KeyCode::Right | KeyCode::Down) => {
                self.menu_state.next_option(MenuOption::COUNT - 1);
            }
            (_, KeyCode::Left | KeyCode::Up) => {
                self.menu_state.previous_option(MenuOption::COUNT - 1);
            }
            _ => {}
        }

        PageEvent::Nothing
    }
}

#[derive(Debug, Clone, EnumCount, VariantArray, IntoStaticStr)]
enum MenuOption {
    Study,
    Configure,
    Quit,
}

impl MenuOption {
    fn to_vec_str<'a>() -> Vec<&'a str> {
        Vec::from(MenuOption::VARIANTS)
            .into_iter()
            .map(MenuOption::into)
            .collect()
    }
}
