use blog_shared::Post;

fn main() {
    let post = Post::new(
        "Post on the server on the api".to_owned(),
        "Let's get rusty on the api!".to_owned(),
    );

    println!("{post:?}");
}
