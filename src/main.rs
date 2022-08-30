use state_machina;
use state_machina::Post;

fn main() {
    let mut post = Post::new();
    println!("When post is empty - {}", post.content());
    post.add_text("Hello. It's me...");
    post.request_review();
    println!("When post isn't approved - {}", post.content());
    post.approve();
    println!("When post is approved {}", post.content());
}