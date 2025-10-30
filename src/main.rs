use std::io;
use std::io::Write;

fn main() {
    println!("───────────────────\n║ MINI-CALCULATOR ║\n───────────────────");
    println!("Select an operation:\n[1] Addition(+)\n[2] Subtraction(-)\n[3] Multiplication(*)\n[4] Division(÷)\n[0] Exit");

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
            1 | 2 | 3 | 4 => todo!(),
            _ => {
                println!("Invalid choice. Choose an operation by typing 0 - 4!")
            }
        }
    }
}

enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Operation {
    fn operate(&self, a: i32, b: i32) -> i32 {
        match self {
            Operation::Addition => a + b,
            Operation::Subtraction => a - b,
            Operation::Multiplication => a * b,
            Operation::Division => a / b,
        }
    }
}