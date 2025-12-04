use crossterm::event::{self, Event, KeyCode};
use ratatui::{Frame, widgets::{self, Borders,Paragraph, Block, HighlightSpacing, List, ListItem, ListState, Padding,
        StatefulWidget, Widget, Wrap,},layout::{Constraint,Direction,Layout,Rect}};
use crate::nitter;

pub fn draw_frame(frame: &mut Frame, app: &mut nitter::App) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(Rect::new(0, 0, 30, 10));

    let items: Vec<ListItem> = app.tweet_list.items
        .iter()
        .map(|tweet| {
            ListItem::new(format!("{} - {}", tweet.header, tweet.author))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Tweets").borders(Borders::ALL));

    frame.render_stateful_widget(list, layout[0], &mut app.tweet_list.state);
}