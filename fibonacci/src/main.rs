fn main() {
    let n: u32 = loop {
        println!("Enter desired Fibonacci's index: ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        match input.trim().parse() {
            Ok(parsed) => break parsed,
            Err(_) => println!("Please enter a positive integer."),
        }
    };

    println!("{n}{} Fibonnaci number = {}", index_suffix(n), fibonacci(n));
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}

fn index_suffix(index: u32) -> &'static str {
    match index {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }
}
