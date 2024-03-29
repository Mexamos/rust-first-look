extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the guessing game!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess..");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("failed to read line!");

        println!("Your guessed: {}", guess);

        let guess: i32 = guess.trim().parse().expect("Type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    };
}
