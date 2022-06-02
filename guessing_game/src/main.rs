use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let x = 2;
    let mut y = x;
    y = y + 1;
    println!("x = {}, y = {}", x, y);
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is {}", secret_number);
    let mut res: bool = false;
    for trial in 1..10 {
        println!("Trial {}: Please input your guess.", trial);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                res = true;
                break;
            }
        }
    }
    if res == false {
        println!("Running out of trials! Better luck next time.");
    }
}
