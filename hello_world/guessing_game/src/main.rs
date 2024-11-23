use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is: {secret_number}");

    loop {
        println!("Please input the guess:");

    // let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    // here, we store values with a variable
    // In rust, variables are immutable by default.
    // the mut is added to make the variable, guess, mutable
    // we are creating a new, empty instance of a string
    let mut guess = String::new();

    // the std::io::stdin()  function returns an instance of std::io::stdin,
    // this is a type that represents a handle to the standard input for the terminal
    io::stdin()
        // the & indicates the argument is a reference - they are immutable by default.
        // this allows multiple parts of the code to access one piece of data without needing to copy that data into memory multiple times.
        .read_line(&mut guess) // calls the read_line method on the stdin handle to get input from the user, &mut guess is the argument to read_line to tell it what string to store the user input in
        // the above returns a Result value which has various possible states called variants - ok and err
        // Ok indicates the operation was successful and inside Ok is the successfully generated value.
        // Err is a variant that means the operation failed and it also contains information about how and why the operation failed.
        .expect("Failed to read line"); //
                                        // {} is a placehol;der

    // shadowing cargo fmt
    // the trim method on a string eliminates the white space at the beginning and the end of the line
    let guess: u32 = match guess
        .trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // this converts a string to another type

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
        
    }
    }
    
}
// Generating a secret number ca
