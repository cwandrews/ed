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

    pub fn add_lines(&mut self, line_num: i32, lines: Vec<String>) {
        let mut ln = line_num;
        for line in lines {
            self.entries.push(Entry::new(ln, Difference::Plus, line));
            ln += 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_has_lines() {
        let mut diff = Diff::new();
        let lines = vec![
            "Here is a line".to_string(),
            "and another".to_string(),
            "and one more".to_string(),
            ];
        diff.add_lines(1, lines);
        assert_eq!(diff.entries.len(), 3);
    }

}
