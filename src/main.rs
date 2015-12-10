mod diff;

use std::io;
use std::io::Write;

struct Range {
    start: i32,
    end: i32,
}

fn input() -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .ok()
            .expect("Error reading line");
        match line.trim() {
            "." => {
                break
            },
            _ => {
                lines.push(line)
            }
        }
    }
    return lines;
}

fn command(line: &String, line_num: &mut i32, diff: &mut diff::Diff) {
    // parse the command line to get
    // Line num or range
    // command
    match line.trim() {
        "a" => {
            append_after(Range{start: *line_num, end: *line_num}, diff);
        },
        "q" => std::process::exit(0),
        _ => println!("Command: {}", line),
    }
}

fn append_after(range: Range, diff: &mut diff::Diff) {
    diff.add_lines(range.end+1, input());
}

fn main() {
    let prompt = ":";
    let mut diff = diff::Diff::new();
    let mut line_num: i32 = 1;
    loop {
        print!("{}", prompt);
        io::stdout().flush();
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => (),
            Err(_) => {
                println!("?");
                continue
            },
        }
        command(&line, &mut line_num, &mut diff);
    }
}
