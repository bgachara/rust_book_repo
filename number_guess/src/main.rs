use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the Number!!");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Your secret number is: {}", secret_number);

    println!("Input your number:");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line.");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a Number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => println!("Alright SmartAss, Fuck Off!!!!")
    }
    // if (guess = 0) {
    //     println!(" Too Loow Brav!! ");
    // }

}
