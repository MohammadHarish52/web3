use std::io;

enum TweetType {
    Regular,
    Excited,
    Question,
}

let tweet_meta = TweetMeta {
    original: tweet.clone(),
    tweet_type,
    length: tweet.len(),
};


fn add_emoji(tweet: &mut String) {
    tweet.push_str(" 😂");
}

fn analyse_tweet(tweet: &String) -> TweetType {
    if tweet.contains("!") {
        return TweetType::Excited;
    } else if tweet.contains("?") {
        return TweetType::Question;
    }
    return TweetType::Regular;
}
fn print_previw(tweet: &String) {
    let preview = &tweet[..std::cmp::min(10, tweet.len())];
    println!("Preview : {}", preview);
}

fn calculate_len(tweet: &String) {
    let len = tweet.len();
    println!("Length : {}", len)
}

fn main() {
    println!("Enter your tweet:");

    let mut tweet = String::new(); // mutable string to store tweet

    io::stdin()
        .read_line(&mut tweet) // read from standard tweet
        .expect("Failed to read line");

    // Remove trailing newline characters
    let mut tweet = tweet.trim().to_string(); // returns a &str

    println!("Original: {}", tweet);
    calculate_len(&tweet);
    print_previw(&tweet);
    add_emoji(&mut tweet);
    let tweet_type = analyse_tweet(&tweet);

    match tweet_type {
        TweetType::Excited => println!("Tweet is excited"),
        TweetType::Question => println!("Tweet is a Question."),
        TweetType::Regular => println!("Tweet is Regular."),
    }

  
}
