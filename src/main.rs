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

fn input() -> Result<Vec<String>, io::Error> {
    let mut lines: Vec<String> = Vec::new();
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(error) => return Err(error),
            Ok(_) => {},
        }
        match line.trim() {
            "." => {
                break
            },
            _ => {
                lines.push(line)
            }
        }
    }
    Ok(lines)
}

fn command(line: &String, line_num: &mut i32, diff: &diff::Diff) -> Result<diff::Diff, String> {
    // parse the command line to get
    // Line num or range
    // command
    match line.trim() {
        "a" => {
            append_after(Range{start: *line_num, end: *line_num}, diff)
                .map_err(|err| err.to_string())
        },
        "q" => std::process::exit(0),
        "u" => Ok(diff.undo()),
        _ => Err("?".to_string()),
    }
}

fn append_after(range: Range, diff: &diff::Diff) -> Result<diff::Diff, io::Error> {
    input()
        .map_err(|err| err)
        .map(|lines| {
            diff.add_lines(range.end+1, lines)
        })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("p", "prompt", "prompt to show while in command mode", "PROMPT");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => panic!(f.to_string())
    };

    let prompt = matches.opt_str("p")
        .unwrap_or_else(|| "".to_string());
    let mut diff = diff::Diff::new();
    let mut line_num: i32 = 1;
    loop {
        print!("{}", prompt);
        match io::stdout().flush() {
            Ok(_) => (),
            Err(err) => {
                println!("Error flushing stdout: {}", err);
                continue
            }
        }
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => (),
            Err(_) => {
                println!("?");
                continue
            },
        }
        match command(&line, &mut line_num, &mut diff) {
            Ok(d) => {
                diff = d;
            },
            Err(err) => println!("Error: {}", err),
            
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
}
