use std::io;

fn main() {
    println!("Let us guess a number :) !");
    println!("Enter your guess here:");

    //Mutable Variable
    //In full, the let mut guess = String::new(); 
    //line has created a mutable variable that is currently bound to a new,
    //empty instance of a String
    let mut guess = String::new();

    // Get input from user 
    io::stdin()
        .read_line(&mut guess) 
        .expect("Failed to read line :(");

        // 
        println!("You guessed: {guess}");
}
