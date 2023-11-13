struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let tweet: Tweet = Tweet {
        username: String::from("Liakos"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
