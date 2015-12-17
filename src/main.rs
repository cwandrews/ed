extern crate getopts;

mod diff;

use std::io;
use std::io::Write;
use std::env;
use getopts::Options;

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
    let lines = input();
    let mut ln = range.end+1;
    for line in lines {
        diff.add_line(ln, line);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("p", "prompt", "prompt to show while in command mode", "PROMPT");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let prompt = matches.opt_str("p")
        .unwrap_or_else(|| "".to_string());
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
