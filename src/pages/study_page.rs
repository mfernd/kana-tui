use crate::app::App;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::Paragraph,
    Frame,
};

pub fn render(_app: &mut App, frame: &mut Frame) {
    let layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(Rect::new(0, 0, frame.area().width, frame.area().height));

    frame.render_widget(Paragraph::new("foo").on_red(), layout[0]);
    frame.render_widget(Paragraph::new("bar").on_blue(), layout[1]);
}
