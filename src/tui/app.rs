pub struct App {
    pub username: String,
    pub sidebar_title: String,
    pub content_title: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            username: String::from("Test"),
            sidebar_title: String::from("sidebar_title content from app.rs goes here"),
            content_title: String::from("content_title content from app.rs goes here"),
        }
    }
}
