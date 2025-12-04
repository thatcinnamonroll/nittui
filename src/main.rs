mod window;
pub mod nitter;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{Frame, widgets::{self, Borders,Paragraph, Block, HighlightSpacing, List, ListItem, ListState, Padding,
        StatefulWidget, Widget, Wrap,},layout::{Constraint,Direction,Layout,Rect}};

fn main() {
    let mut terminal = ratatui::init();

    let tweets = vec![
        nitter::tweet { header: "Hello World".to_string(), author: "Alice".to_string(), likes: 5, comments: 2, is_retweet: false },
        nitter::tweet { header: "Rust is great".to_string(), author: "Bob".to_string(), likes: 10, comments: 3, is_retweet: true },
    ];
    let mut app = nitter::App {
            tweet_list: nitter::TweetList {
            items: tweets,
            state: ListState::default(),
        }
        };
        // this 0 is to select the first item in list
        app.tweet_list.state.select(Some(0));
    loop {
        terminal.draw(|f|window::draw_frame(f,&mut app)).expect("failed to draw frame");
        if let Ok(Event::Key(key)) = event::read(){
            match key.code {
                    KeyCode::Up => app.tweet_list.state.select_previous(),                  
                    KeyCode::Down => app.tweet_list.state.select_next(),                    
                    KeyCode::End => break,
                    _ => {} 
            }
        }
    } 
    ratatui::restore();
}
