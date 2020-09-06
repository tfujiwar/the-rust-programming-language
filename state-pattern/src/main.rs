fn main() {
    let mut post = Post::new();
    post.add_content("This is a content.");
    println!("Draft: {}", post.content());

    post.request_review();
    println!("Requested: {}", post.content());

    post.approve();
    println!("Approved: {}", post.content());
}

struct Post {
    content: String,
    state: Option<Box<dyn State>>,
}

impl Post {
    fn new() -> Post {
        Post {
            content: String::new(),
            state: Some(Box::new(Draft {})),
        }
    }

    fn add_content(&mut self, s: &str) {
        self.content.push_str(s);
    }

    fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a> (&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(Pending {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct Pending {}

impl State for Pending {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Approved {})
    }
}

struct Approved {}

impl State for Approved {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a> (&self, post: &'a Post) -> &'a str {
        &post.content
    }
}