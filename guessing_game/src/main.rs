use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("wanna guess the number?");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("give me your guess uwu");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read input :(\nwhat the fuck did you do??!? ;-;");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
