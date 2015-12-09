use std::io;
use std::io::Write;

fn main() {
    let prompt = ":";
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => (),
            Err(_) => {
                println!("?");
                continue
            }
        }
        println!("You typed in {}", line.trim());
    }
}
