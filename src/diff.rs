use std::vec::Vec;

enum Difference {
    Minus,
    Plus,
}

struct Entry {
    line_num: i32,
    difference: Difference,
    line: String,
}

impl Entry {
    fn new(line_num: i32, difference: Difference, line: String) -> Entry {
        Entry {
            line_num: line_num,
            difference: difference,
            line: line,
        }
    }
}

pub struct Diff {
    entries: Vec<Entry>,
}

impl Diff {
    pub fn new() -> Diff {
        let mut entries: Vec<Entry> = Vec::new();
        Diff {
            entries: entries,
        }
    }

    pub fn add_line(&mut self, line_num: i32, line: String) {
        self.entries.push(Entry::new(line_num, Difference::Plus, line));
    }

    pub fn del_line(&mut self, line_num: i32, line: String) {
        self.entries.push(Entry::new(line_num, Difference::Minus, line));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_diff_empty() {
        let mut diff = Diff::new();
        assert!(diff.entries.is_empty());
    }

    #[test]
    fn diff_has_lines() {
        let mut diff = Diff::new();
        diff.add_line(1, "Here is a line".to_string());
        diff.add_line(2, "and another".to_string());
        diff.add_line(3, "and one more".to_string());
        assert_eq!(diff.entries.len(), 3);
    }
}
