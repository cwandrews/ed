use std::io;

enum Mode {
    Command,
    Input,
}

struct Range {
    start: i32,
    end: i32,
}


fn command_line(line: &String, mode: &mut Mode) {
    match line.trim() {
        "a" => *mode = Mode::Input,
        "q" => std::process::exit(0),
        _ => println!("Command: {}", line),
    }
}

fn input_line(line: &String, mode: &mut Mode) {
    match line.trim() {
        "." => *mode = Mode::Command,
        _ => println!("Input: {}", line),
    }
}

fn append_after(lines: Vec<line>, range: Range, diff: &Diff) {
    diff.add_lines(range.start, lines);
}

fn main() {
    let mut mode = Mode::Command;
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => (),
            Err(_) => {
                println!("?");
                continue
            },
        }
        match mode {
            Mode::Command => command_line(&line, &mut mode),
            Mode::Input => input_line(&line, &mut mode),
        }
    }
}
