use ratatui::widgets::ListState;

pub struct App {
    pub tweet_list: TweetList,
    pub opened_tweet: Option<tweet>,
}

pub struct TweetList  {
    pub items: Vec<tweet>,
    pub state: ListState,
}

#[derive(Debug, Clone)]
pub struct tweet {
    pub header: String,
    pub author: String,
    pub content: String,
    pub likes: i32,
    pub comments: i32,
    pub is_retweet: bool,
}