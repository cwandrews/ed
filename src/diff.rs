use std::vec::Vec;

enum Difference {
    Minus,
    Plus,
}

struct diff_entry {
    line_num: i32,
    difference: Difference,
    line: String,
}

impl diff_entry {
    fn new(line_num: i32, difference: Difference, line: String) -> diff_entry {
        diff_entry {
            line_num: line_num,
            difference: difference,
            line: line,
        }
    }
}

struct Diff {
    entries: Vec<diff_entry>,
}

impl Diff {
    fn new() -> Diff {
        let mut entries: Vec<diff_entry> = Vec::new();
        Diff {
            entries: entries,
        }
    }

    fn add_lines(&self, line_num: i32, lines: Vec<String>) {
        let mut ln = line_num;
        for line in lines {
            self.entries.push(diff_entry::new(ln, Difference::Plus, line));
            ln += 1
        }
    }
}
