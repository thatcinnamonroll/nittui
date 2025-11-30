use crossterm::event::{self, Event, KeyCode};
use ratatui::{Frame, widgets::{self, Borders,Paragraph, Block, HighlightSpacing, List, ListItem, ListState, Padding,
        StatefulWidget, Widget, Wrap,},layout::{Constraint,Direction,Layout,Rect}};

struct App {
    tweet_list: TweetList,
}

struct TweetList  {
    items: Vec<tweet>,
    state: ListState,
}

#[derive(Debug)]
struct tweet {
    header: String,
    author: String,
    likes: i32,
    comments: i32,
    is_retweet: bool,
}

pub fn show_window() {
    let mut terminal = ratatui::init();

    let tweets = vec![
        tweet { header: "Hello World".to_string(), author: "Alice".to_string(), likes: 5, comments: 2, is_retweet: false },
        tweet { header: "Rust is great".to_string(), author: "Bob".to_string(), likes: 10, comments: 3, is_retweet: true },
    ];
    let mut app = App {
            tweet_list: TweetList {
            items: tweets,
            state: ListState::default(),
        }
        };
        app.tweet_list.state.select(Some(0));
    loop {
        terminal.draw(|f|draw(f,&mut app)).expect("failed to draw frame");
        if let Ok(Event::Key(key)) = event::read(){
            match key.code {
                    KeyCode::Up => {
                        let i = app.tweet_list.state.selected().unwrap_or(0);
                        let new = if i == 0 { app.tweet_list.items.len() - 1 } else { i - 1 };
                        app.tweet_list.state.select(Some(new));
                    }
                    KeyCode::Down => {
                        let i = app.tweet_list.state.selected().unwrap_or(0);
                        let new = if i >= app.tweet_list.items.len() - 1 { 0 } else { i + 1 };
                        app.tweet_list.state.select(Some(new));
                    }
                    KeyCode::End => break,
                    _ => {} 
            }
        }
    } 
    ratatui::restore();
}

fn draw(frame: &mut Frame, app: &mut App) {
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