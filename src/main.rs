mod window;
pub mod nitter;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{Frame, layout::{Constraint,Direction,Layout,Rect}, symbols::braille, widgets::{self, Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph, StatefulWidget, Widget, Wrap}};

use crate::nitter::load_tweets;

fn main() {
    let mut terminal = ratatui::init();

    // some dummy data
    let tweets = load_tweets();
    let mut app = nitter::App {
            opened_tweet: None,
            tweet_list: nitter::TweetList {
            items: tweets,
            state: ListState::default(),
        }
        };
        // this 0 is to select the first item in list
        app.tweet_list.state.select(Some(0));
    loop {
        terminal.draw(|f| {
            window::draw_ui(f, &mut app);
        }).unwrap();
        if let Ok(Event::Key(key)) = event::read(){
            match key.code {
                    KeyCode::Up => app.tweet_list.state.select_previous(),                  
                    KeyCode::Down => app.tweet_list.state.select_next(),     
                    KeyCode::Enter => {
                        if let Some(i) = app.tweet_list.state.selected() {
                            app.opened_tweet = app.tweet_list.items.get(i).cloned();
                        }
                    },
                    KeyCode::End => break,
                    KeyCode::Esc => app.opened_tweet = None,
                    _ => {} 
            }
        }
    } 
    ratatui::restore();
}
