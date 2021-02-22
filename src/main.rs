use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let number = rand::thread_rng().gen_range(1..101);

    println!("Guess the number!");
    println!("Please input your guess.");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number, please write numeric");
                continue;
            },
        };

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
