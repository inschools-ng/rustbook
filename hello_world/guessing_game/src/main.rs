use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input the guess:");

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
    println!("You guessed: {guess}");
}
