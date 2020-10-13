//importing the io library from the stardand library
use std::io;

fn main() {
    println!("Gues the number!");
    println!("Please input your guess.");
    //create a mutable string guess and pass the instance of a new string
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); //read an input from a user, pass it as a mutable reference 
        //& handle potential failer of the program
    println!("You guessed: {}", guess); //print the value of guess
}

