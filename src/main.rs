use std::io;
use std::io::Write;

fn main() {
    println!("───────────────────\n║ MINI-CALCULATOR ║\n───────────────────");
    println!("Select an operation:\n[1] Addition(+)\n[2] Subtraction(-)\n[3] Multiplication(*)\n[4] Subtraction(÷)\n[0] Exit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(input) => input,
            Err(e) => {
                println!("Failed to read line: {e}");
                continue;
            }
        };

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{e}. Please enter a number!");
                continue;
            }
        };

        match input {
            0 => break,
            _ => todo!()
        }
    }
}
