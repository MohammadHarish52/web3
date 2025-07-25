fn parse_tweet(tweet: &str) -> Result<usize, String> {
    if tweet.len() >= 280 {
        Err(String::from("Tweet is above 280 characters"))
    } else {
        Ok(tweet.len())
    }
}

fn main() {
    let tweet =
        String::from("Hey there i am thinking about doin 70 hr week can i do it i really want to");
    match parse_tweet(&tweet) {
        Ok(length) => println!("The length is {}", length),
        Err(msg) => println!("Error is {}", msg),
    }
}
