extern crate getopts;

mod diff;

use std::io;
use std::io::Write;
use std::env;
use getopts::Options;

static CurrentAddress: Address = Address::Nth(0);

struct Range {
    start: Box<Address>,
    end: Box<Address>,
}

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

trait Operation {
    fn apply(Address, &diff::Diff) -> Result<diff::Diff, String>;
}

struct nop;

impl Operation for nop {
    fn apply(addr: Address, diff: &diff::Diff) -> Result<diff::Diff, String> {
        panic!("Command didn't get set. This shouldn't have happened");
        Err("Command didn't get set. This shouldn't have happened".to_string())
    }
}

struct append_after;

impl Operation for append_after {
    fn apply(addr: Address, diff: &diff::Diff) -> Result<diff::Diff, String> {
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
}

struct undo;

impl Operation for undo {
    fn apply(addr: Address, diff: &diff::Diff) -> Result<diff::Diff, String> {
        Ok(diff.undo())
    }
}

struct quit;

impl Operation for quit {
    fn apply(addr: Address, diff: &diff::Diff) -> Result<diff::Diff, String> {
        std::process::exit(0);
        Err("Should never get here".to_string())
    }
}

struct Command {
    address: Address,
    operation: Operation,
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
        address: Address::Current,
        command: nop,
    };
    command.command = match line.trim() {
        "a" => append_after,
        "q" => quit,
        "u" => undo,
        _ => {
            return Err("Uknown command".to_string())
        },
    };
    Ok(command)
}

fn command(line: &String, line_num: i32, diff: &diff::Diff) -> Result<diff::Diff, String> {
    parse_cmd(line)
        .map_err(|err| err.to_string())
        .and_then(|cmd| {
            cmd.operation.apply(cmd.address, diff)
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
        match command(&line, line_num, &mut diff) {
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
