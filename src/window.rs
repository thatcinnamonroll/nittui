use crossterm::event::{self, Event, KeyCode};
use ratatui::{Frame, layout::{self, Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style, Stylize, palette::tailwind::{BLUE, GREEN, SLATE}}, widgets::{self, Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph, StatefulWidget, Widget, Wrap}};
use crate::nitter::{self, tweet};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub fn draw_list(frame: &mut Frame, area: Rect, app: &mut nitter::App) {
    let items: Vec<ListItem> = app.tweet_list.items
        .iter()
        .map(|tweet| {
            ListItem::new(format!("{} \n {} \n Likes:{} Retweet:{}", tweet.header, tweet.author,tweet.likes,tweet.is_retweet))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Tweets").borders(Borders::ALL)).highlight_style(SELECTED_STYLE);

    frame.render_stateful_widget(list, area, &mut app.tweet_list.state);
}

pub fn draw_tweet_view(frame: &mut Frame, area: Rect, tweet_data: Option<&nitter::tweet>) {
    let tweet_content_block = Block::default()
        .title("Tweet")
        .borders(Borders::ALL);

    if let Some(t) = tweet_data {
        let tweetContent = &t.content;
        let tweetLikes = t.likes;
        let isRetweet = t.is_retweet;
        let tweetAuthor = &t.author;
        let tweetComentsCount = t.comments;

        let tweet_paragraph = Paragraph::new(format!(
            "{}\nLikes:{}  Comments:{}  Retweet:{}\n\n{}",
            tweetAuthor, tweetLikes, tweetComentsCount, isRetweet, tweetContent
        ))
        .block(tweet_content_block);

        frame.render_widget(tweet_paragraph, area);

    } else {
        frame.render_widget(
            Paragraph::new("Press Enter to open tweet").block(tweet_content_block),
            area,
        );
    }
}

pub fn draw_ui(frame: &mut Frame, app: &mut nitter::App) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.size());

    draw_list(frame, layout[0], app);
    draw_tweet_view(frame, layout[1],  app.opened_tweet.as_ref());
}
