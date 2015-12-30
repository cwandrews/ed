trait Buffer {
    fn length(&self) -> i32;
    fn insert(&self, line_num: i32, line: String) -> Result
}
