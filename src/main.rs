use std::env;

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        n + fibonacci(n - 1)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide a number as an argument.");
        return;
    }

    let n: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Please provide a valid integer.");
            return;
        }
    };

    let result = fibonacci(n);
    println!("Fibonacci({}) = {}", n, result);
}