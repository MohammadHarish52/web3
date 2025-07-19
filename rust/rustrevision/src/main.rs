// enum with pattern matching
enum TweetType {
    Text(String),
    Image(String),
    Video { url: String, duration: u32 },
}

fn analyze_tweet(tweet: TweetType) {
    match tweet {
        TweetType::Text(tweet) => println!("Tweet is {}", tweet),
        TweetType::Image(img) => println!("image is {}", img),
        TweetType::Video { url, duration } => {
            println!("Video url is {} and is {} long", url, duration)
        }
    }
}

fn main() {
    analyze_tweet(TweetType::Text(String::from(
        "Rust is such a great language",
    )));
    analyze_tweet(TweetType::Image(String::from("Some ur")));
    analyze_tweet(TweetType::Video {
        url: (String::from("some url")),
        duration: (10),
    });
}
