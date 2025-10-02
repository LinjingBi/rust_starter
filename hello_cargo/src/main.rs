use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    // main_stdio_test();
    guess_secret_number();
}

fn main_stdio_test() {
    println!("Hello!");
    println!("Guess a number:");
    // printlin! is a macro, not a normal function like println, it doesn't always follow the same rules as functions.
    let mut guess = "8".to_string();
    io::stdin()
        .read_line(&mut guess) // & is a reference, save memory??
        .expect("Failed to read from input.");
    println!("Your guess: {guess}"); //82, because read_line only append to &mut, not overwrite.


}

fn guess_secret_number() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number {secret_number}"); 
    loop{

        let mut guess = String::new();
        println!("Guess a number:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read from input.");
        println!("Your guess {guess}");

        // let guess: u32 = guess.trim().parse().expect("invalid number.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Number only.");
                continue;
            }
        };

        if secret_number < guess {
            println!("smaller");
        } else if secret_number > guess {
            println!("bigger");
        } else {
            println!("exact match");
            break;
        }

        
        // match secret_number.cmp(&guess) {
        //     Ordering::Less => println!("decrease"),
        //     Ordering::Greater => println!("increase"),
        //     Ordering::Equal => {
        //             println!("exact right");
        //             break;
        //         }
        //     }
    }
}