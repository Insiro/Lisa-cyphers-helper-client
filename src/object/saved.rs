trait SavedFactory {
    fn save(&mut self);
    fn load(key: &str) -> Self;
    fn new() -> Self;
    fn delete(&mut self);
    fn from_file() -> Self;
}
