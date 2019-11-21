use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("Blah blah blah...");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Blah blah blah...", post.content());
}
