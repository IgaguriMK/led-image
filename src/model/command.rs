#[derive(Debug, Clone)]
pub enum Command {
    Text(Text),
    Space(Space),
}

#[derive(Debug, Clone)]
pub struct Text {
    content: String,
    foreground: Option<String>,
    background: Option<String>,
}

impl Text {
    pub fn new(content: String, foreground: Option<String>, background: Option<String>) -> Text {
        Text {
            content,
            foreground,
            background,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn foreground(&self) -> Option<&str> {
        self.foreground.as_ref().map(String::as_str)
    }

    pub fn background(&self) -> Option<&str> {
        self.background.as_ref().map(String::as_str)
    }
}

#[derive(Debug, Clone)]
pub struct Space {
    width: usize,
    background: Option<String>,
}

impl Space {
    pub fn new(width: usize, background: Option<String>) -> Space {
        Space { width, background }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn background(&self) -> Option<&str> {
        self.background.as_ref().map(String::as_str)
    }
}
