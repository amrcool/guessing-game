use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {
    println!("{}","|| Hello to the guessing game ||".purple());
    let secret_number = rand::thread_rng().gen_range(1..11);

    println!("the secret number: {} ",secret_number);

    // println!("enter your guess here: ");
    // let mut guess = String :: new();
    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("failed to read the line.");

    // let guess : u32 = guess.trim().parse().expect("please enter a number");

    // println!("you guessed: {} ",guess);

    // match guess.cmp(&secret_number){
    //     Ordering::Less=> println!("nuh too small :("),
    //     Ordering::Equal=> println!("you win !"),
    //     Ordering::Greater=> println!("nuh too big :("),
    // }
    loop{
        println!("enter your guess here: ");
        let mut guess = String :: new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read the line.");

        let guess : u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) =>{println!("{}","wrong input please enter a number".red());continue;},

        };

        println!("you guessed: {} ",guess);

        match guess.cmp(&secret_number){
            Ordering::Less=> println!("{}","nuh too small :(".blue()),
            Ordering::Equal=> {println!("{}","correct you win !".green());break;},
            Ordering::Greater=> println!("{}","nuh too big :(".yellow()),
        }

    }

}
