use crossterm::event::{self, Event};
use ratatui::{Frame, widgets::{self, Borders,Paragraph},layout::{Constraint,Direction,Layout,Rect}};

pub fn show_window() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Length(10), Constraint::Min(0)],
    )
    .split(Rect::new(0, 0, 30, 10));
    frame.render_widget(Paragraph::new("foo"), layout[0]);
    frame.render_widget(widgets::Block::default().borders(Borders::all()), layout[1]);
}