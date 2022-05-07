pub trait TextBuffer {
    fn insert(&mut self, loc: usize, text: &str);
    fn delete(&mut self, start: usize, end: usize);
    fn get_text(&self) -> String;
}