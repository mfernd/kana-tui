use crate::app::App;
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Paragraph},
    Frame,
};

pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!("kana: {:?}", &app.study_plan))
            .block(
                Block::bordered()
                    .title("kana-tui")
                    .title_alignment(Alignment::Center)
                    .style(Style::new().fg(Color::Red)),
            )
            .style(Style::new().fg(Color::Reset))
            .centered(),
        frame.area(),
    )
}
