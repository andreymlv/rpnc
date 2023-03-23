use std::io::{self, Write};

fn main() -> std::io::Result<()> {
    // TODO: Read input file and evaluate.
    // TODO: Read input expression and evaluate.
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let result = calculate(input.trim());
        println!("{:?}", result);
    }
}

fn calculate(trim: &str) -> Vec<f32> {
    let splitted_by_space: Vec<&str> = trim.split(" ").collect();
    let evaluated: Vec<f32> = evaluate(splitted_by_space);
    evaluated
}

fn evaluate(splitted_by_space: Vec<&str>) -> Vec<f32> {
    let mut stack: Vec<f32> = vec![];
    for i in splitted_by_space {
        match i.parse::<f32>() {
            Ok(i) => stack.push(i),
            Err(_) => {
                let y = stack.pop().unwrap();
                let x = stack.pop().unwrap();
                // TODO: Write enumartion with operators. Then write macros that generates that
                // code:
                if i == "+" {
                    stack.push(x + y);
                } else if i == "-" {
                    stack.push(x - y);
                } else if i == "/" {
                    if y == 0.0 {
                        eprintln!("Can't divide by zero.");
                        break;
                    }
                    stack.push(x / y);
                } else if i == "*" {
                    stack.push(x * y);
                }
            }
        }
    }
    stack
}
