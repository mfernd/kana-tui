use crate::app::{App, Page};
use crate::event::EventHandler;
use crate::{pages, AppResult};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::Backend;
use ratatui::layout::{Alignment, Constraint, Flex, Layout, Margin, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders};
use ratatui::{Frame, Terminal};
use std::io;
use std::panic;

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
#[derive(Debug)]
pub struct Tui<B: Backend> {
    /// Interface to the Terminal.
    terminal: Terminal<B>,
    /// Terminal event handler.
    pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    /// Initializes the terminal interface.
    ///
    /// It enables the raw mode and sets terminal properties.
    pub fn init(&mut self) -> AppResult<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

        // Define a custom panic hook to reset the terminal properties.
        // This way, you won't have your terminal messed up if an unexpected error happens.
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    /// Draw the terminal interface by [`rendering`] the widgets.
    /// Corresponding to each page render function.
    pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
        self.terminal.draw(|frame| {
            let main_area = flex(
                frame.area(),
                (Flex::Center, Constraint::Max(50)),
                (Flex::Center, Constraint::Max(30)),
            );
            render_header_block(frame, main_area);
            // to prevent overlap with the header block
            let inner_main_area = main_area.inner(Margin::new(1, 1));
            match app.current_page {
                Page::Homepage(_) => pages::homepage::render(app, frame, inner_main_area),
                Page::StudyPage(_) => pages::study_page::render(app, frame, inner_main_area),
            }
        })?;
        Ok(())
    }

    /// Resets the terminal interface.
    ///
    /// This function is also used for the panic hook to revert
    /// the terminal properties if unexpected errors occur.
    fn reset() -> AppResult<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    /// Exits the terminal interface.
    ///
    /// It disables the raw mode and reverts back the terminal properties.
    pub fn exit(&mut self) -> AppResult<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

fn render_header_block(frame: &mut Frame, main_area: Rect) {
    let block = Block::default()
        .title(" kana-tui ")
        .title_alignment(Alignment::Center)
        .title_style(Style::default().bold().fg(Color::Red))
        .borders(Borders::all())
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(Color::DarkGray));

    frame.render_widget(block, main_area);
}

pub fn flex(area: Rect, horizontal: (Flex, Constraint), vertical: (Flex, Constraint)) -> Rect {
    let [area] = Layout::horizontal([horizontal.1])
        .flex(horizontal.0)
        .areas(area);
    let [area] = Layout::vertical([vertical.1]).flex(vertical.0).areas(area);
    area
}
