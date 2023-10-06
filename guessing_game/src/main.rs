use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("The GUESSING Game");
    println!("-----------------");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("Sssshhhh 🤫 [{secret_number}]");

    loop {
        println!("\nTake a guess [1,100]: ");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        // println!("Your input: {input}");
        let guessed_number: u32 = match input.trim().parse() {
            Ok(parsed_number) => parsed_number,
            Err(_) => {
                println!("Please enter a positive integer.");
                continue;
            }
        };

        match guessed_number.cmp(&secret_number) {
            Ordering::Less => println!("Nope... Try higher!"),
            Ordering::Greater => println!("Arg... Go lower!"),
            Ordering::Equal => {
                println!("This is a WIN!");
                break;
            }
        }
    }
}
