use crate::shell::Shell;

pub fn execute(_shell: &mut Shell, args: &[&str]) {
    if args.len() < 3 {
        println!("Usage: calc <num1> <op> <num2>");
        return;
    }

    let num1: f64 = match args[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number: {}", args[0]);
            return;
        }
    };

    let op = args[1];

    let num2: f64 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number: {}", args[2]);
            return;
        }
    };

    let result = match op {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Division by zero");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("Unknown operator: {}", op);
            return;
        }
    };

    println!("{} {} {} = {}", num1, op, num2, result);
}
