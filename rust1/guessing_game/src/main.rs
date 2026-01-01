use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess");

    let secret_number = rand::rng().random_range(1..=100);

    println!("your  guess");

    

    loop {
        println!("Please guess a number: ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too bif"),
            Ordering::Equal => {
                println!("got it");
                break;
            },
        }

    }
    
}