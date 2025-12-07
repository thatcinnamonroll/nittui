use ratatui::widgets::ListState;

pub struct App {
    pub tweet_list: TweetList,
    pub opened_tweet: Option<tweet>,
}

pub struct TweetList  {
    pub items: Vec<tweet>,
    pub state: ListState,
}

#[derive(Debug,Clone)]
pub struct tweet {
    pub header: String,
    pub author: String,
    pub content: String,
    pub likes: i32,
    pub commentsCounter: i32,
    pub comments: Vec<comment>,
    pub is_retweet: bool,
}

#[derive(Debug,Clone)]
pub struct comment {
    pub author: String,
    pub content: String,
    pub likes: i32,
    pub commentsCounter: i32,
    // nah i'm not making comments of comments :\ there will be way to open nitter in users browser
}

pub fn load_tweets() -> Vec<tweet> {
    // i know this is mess but it's only placeholder, after i'm done with ui and start working on rss i will make it look better :)
    let first_comment = vec![comment{author:"Josh".to_string(),content:"Hi!!!!".to_string(),likes:2,commentsCounter:0}];
    let second_comment = vec![comment{author:"Lucas".to_string(),content:"true :)".to_string(),likes:10,commentsCounter:3},comment{author:"Emma".to_string(),content:"i also like rust!".to_string(),likes:30,commentsCounter:2}];

    return vec![
        tweet { header: "Hello World".to_string(), author: "Alice".to_string(),content: "Hello to nittui".to_string(), likes: 5, commentsCounter: 2,comments:first_comment, is_retweet: false },
        tweet { header: "Rust is great".to_string(), author: "Bob".to_string(),content: "Rust is so cool :D".to_string(), likes: 10, commentsCounter: 3,comments:second_comment, is_retweet: true },
    ];
}