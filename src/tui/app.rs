pub struct App {
    pub username: String,
    pub content: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            username: String::from("Test"),
            content: String::from("Content test"),
        }
    }
}
