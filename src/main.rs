extern crate getopts;

mod diff;
mod buffer;

use std::io;
use std::io::Write;
use std::env;
use getopts::Options;

static CurrentAddress: Address = Address::Nth(1);

#[derive(Clone)]
struct Range {
    start: Box<Address>,
    end: Box<Address>,
}

#[derive(Clone)]
enum Address {
    Current,
    Last,
    Nth(i32),
    Previous,
    NthPrevious(i32),
    Next,
    NthNext(i32),
    All,
    Range(Range),
    CurrentToEnd,
    RENext(String),
    REPrevious(String),
    Mark(String),
}

fn append_after(addr: Address, diff: &diff::Diff) -> Result<diff::Diff, String> {
    let line_num: i32 = match addr {
        Address::Nth(ln) => ln,
        _ => 0,
    };
    input()
        .map_err(|err| err.to_string())
        .map(|lines| {
            diff.add_lines(line_num, lines)
        })
}

fn undo(addr: Address, diff: &diff::Diff) -> Result<diff::Diff, String> {
    Ok(diff.undo())
}

fn quit(addr: Address, diff: &diff::Diff) -> Result<diff::Diff, String> {
    std::process::exit(0);
    Err("Should never get here".to_string())
}

fn print(addr: Address, diff: &diff::Diff) -> Result<diff::Diff, String> {
    let fb = buffer::FileBuffer::new(diff.clone(), buffer::BufferFile::NoneGiven);
    match fb.lines() {
        Ok(lines) => {
            for line in lines {
                print!("{}", line);
            }
            io::stdout().flush().unwrap();
            Ok(diff.clone())
        },
        Err(err) => Err(err.to_string()),
    }
}

struct Command {
    address: Address,
    operation: fn(Address, &diff::Diff) -> Result<diff::Diff, String>,
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

fn parse_cmd(line: &String) -> Result<Command, String> {
    let mut command = Command {
        address: CurrentAddress.clone(),
        operation: match line.trim() {
            "a" => append_after,
            "q" => quit,
            "u" => undo,
            "p" => print,
            _ => {
                return Err("Uknown command".to_string())
            },
        }
    };
    Ok(command)
}

fn command(line: &String, diff: &diff::Diff) -> Result<diff::Diff, String> {
    parse_cmd(line)
        .map_err(|err| err.to_string())
        .and_then(|cmd| {
            (cmd.operation)(cmd.address, diff)
        })
}

fn cmd_line(prompt: &String) -> Result<String, String> {
    let mut line = String::new();
    print!("{}", prompt);
    io::stdout().flush()
        .map_err(|err| err.to_string())
        .and_then(|_| {
            io::stdin().read_line(&mut line)
                .map_err(|err| err.to_string())
        })
        .and_then(|_| Ok(line))
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
    loop {
        match cmd_line(&prompt)
            .map_err(|err| err.to_string())
            .and_then(|line| command(&line, &diff)) {
                Ok(d) => diff = d,
                Err(err) => println!("{}", err),
            }
    }
}
