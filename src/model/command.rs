pub trait Command<'a, T, S> {
    fn when_text<U>(&'a self, none: U, f: impl FnOnce(&T) -> U) -> U;
    fn when_space<U>(&'a self, none: U, f: impl FnOnce(&S) -> U) -> U;
}

pub trait Text {
    fn content(&self) -> &str;
    fn foreground(&self) -> Option<&str>;
    fn background(&self) -> Option<&str>;
}

pub trait Space {
    fn width(&self) -> usize;
    fn background(&self) -> Option<&str>;
}
