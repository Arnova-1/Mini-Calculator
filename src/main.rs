use std::io;
use std::io::Write;

fn main() {
    println!("───────────────────\n║ MINI-CALCULATOR ║\n───────────────────");

    loop {
        println!("Select an operation:\n[1] Addition(+)\n[2] Subtraction(-)\n[3] Multiplication(*)\n[4] Division(÷)\n[0] Exit");
        let input: i32 = prompt() as i32;

        if input == 0 {
            println!("\nExiting...\n");
            std::process::exit(0);
        }

        println!("Enter the first number: ");
        let first_number = prompt();

        println!("Enter the second number: ");
        let second_number = prompt();

        match input {
            1 => print_results(&Operation::Addition, first_number, second_number),
            2 => print_results(&Operation::Subtraction, first_number, second_number),
            3 => print_results(&Operation::Multiplication, first_number, second_number),
            4 => print_results(&Operation::Division, first_number, second_number),
            _ => {
                println!("Invalid choice. Choose an operation by typing 0 - 4!");
                continue;
            }
        }
    };
}

fn print_results(operation: &Operation, a: f32, b: f32) {
    let result = Operation::operate(&operation, a, b);
    let symbol = Operation::symbol(&operation);
    println!("============================\nThe result of {a} {symbol} {b} = {result}\n============================");
}
enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Operation {
    fn symbol(&self) -> &'static str {
        match self {
            Operation::Addition => "+",
            Operation::Subtraction => "-",
            Operation::Multiplication => "*",
            Operation::Division => "/"
        }
    }

    fn operate(&self, a: f32, b: f32) -> f32 {
        match self {
            Operation::Addition => a + b,
            Operation::Subtraction => a - b,
            Operation::Multiplication => a * b,
            Operation::Division => a / b,
        }
    }
}

fn prompt() -> f32 {
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

        let input: f32 = match input.trim().parse() {
            Ok(input) => input,
            Err(e) => {
                println!("{e}. Please enter a number!");
                continue;
            }
        };
        return input;
    }
}