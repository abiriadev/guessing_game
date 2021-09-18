use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    intro();

    loop {
        let secret_number = rand::thread_rng().gen_range(1, 101);
        // println!("The secret number is: {}", secret_number);

        match game(secret_number) {
            true => continue,
            false => break,
        }
    }

    println!("\ngood bye! :)");
}

fn intro() {
    println!("Guess the number!");
    println!("I will take a number in the range of 1 to 100.");
    println!("If you want to break, please type 'quit'.");
}

fn game(secret_number: u32) -> bool {
    println!("\nNEW GAME");

    loop {
        println!("\nPlease input your guess: ");

        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(_) => {}
            Err(_) => println!("Failed to read line."),
        };

        let guess = guess.trim();

        // println!("{}", guess);

        match guess.eq("quit") {
            true => return false,
            false => {}
        };

        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        //.expect("Please type a number!");

        // println!("You guessed: {}", guess);

        let max: u32 = 100;
        let min: u32 = 1;

        match guess.cmp(&max) {
            Ordering::Greater => {
                println!("Please in the range of 1 to 100");
                continue;
            }
            _ => {}
        };

        match guess.cmp(&min) {
            Ordering::Less => {
                println!("Please in the range of 1 to 100");
                continue;
            }
            _ => {}
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                return true;
            }
        }
    }
}
