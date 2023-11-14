use traits::{Summary,Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("AAAA"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    notify(&tweet);
}
//约束
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}",item.summarize());
}
