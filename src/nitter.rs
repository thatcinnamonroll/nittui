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

pub fn load_tweets() -> Vec<tweet> {
    return vec![
        tweet { header: "Hello World".to_string(), author: "Alice".to_string(),content: "Hello to nittui".to_string(), likes: 5, comments: 2, is_retweet: false },
        tweet { header: "Rust is great".to_string(), author: "Bob".to_string(),content: "Rust is so cool :D".to_string(), likes: 10, comments: 3, is_retweet: true },
    ];
}