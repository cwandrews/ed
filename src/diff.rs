use std::vec::Vec;

#[derive(Clone)]
enum Difference {
    Minus,
    Plus,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Diff {
    entries: Vec<Entry>,
}

impl Diff {
    pub fn new() -> Diff {
        let entries: Vec<Entry> = Vec::new();
        Diff {
            entries: entries,
        }
    }

    pub fn add_line(&self, line_num: i32, line: String) -> Diff {
        let mut new_diff = self.clone();
        new_diff.entries.push(Entry::new(line_num, Difference::Plus, line));
        new_diff
    }

    pub fn add_lines(&self, line_num: i32, lines: Vec<String>) -> Diff {
        let mut new_diff = self.clone();
        let mut ln = line_num;
        for line in lines {
            new_diff.entries.push(Entry::new(ln, Difference::Plus, line));
            ln += 1;
        }
        new_diff
    }

    pub fn del_line(&self, line_num: i32, line: String) -> Diff {
        let mut new_diff = self.clone();
        new_diff.entries.push(Entry::new(line_num, Difference::Minus, line));
        new_diff
    }

    pub fn undo(&self) -> Diff  {
        let mut new_diff = self.clone();
        new_diff.entries.pop();
        new_diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_diff_empty() {
        let diff = Diff::new();
        assert!(diff.entries.is_empty());
    }

    #[test]
    fn diff_has_lines() {
        let mut  diff = Diff::new();
        diff = diff.add_line(1, "Here is a line".to_string());
        diff = diff.del_line(1, "Here is a line".to_string());
        diff = diff.add_line(2, "and one more".to_string());
        assert_eq!(diff.entries.len(), 3);
    }

    #[test]
    fn diff_undo() {
        let mut diff = Diff::new();
        diff = diff.add_line(1, "Here is a line".to_string());
        diff = diff.undo();
        assert!(diff.entries.is_empty());
    }
}
