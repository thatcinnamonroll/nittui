use crossterm::event::{self, Event};
use ratatui::{Frame, text::Text, widgets::{self, Borders}};

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
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
    let test_widget = widgets::Block::default().borders(Borders::all());
    frame.render_widget(test_widget, frame.area());
}