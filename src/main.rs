use std::io; //importing the standard library
use rand::Rng; //importing the rand library 
use std::cmp::Ordering; //importing the Ordering

fn main() {

    println!("Guess the number!");

    //create a variable secret_number and generate a random number between 1 and 100 and assign into that varibale
    let secret_number  = rand::thread_rng().gen_range(1, 101);
    
    loop{ 
        println!("Please input your guess."); //printing to thr screen using or calling a macro
    
        //create a mutable string guess and pass the instance of a new string
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); //read an input from a user, pass it as a mutable reference 
            //& handle potential failer of the program
    
        //u32 - unassigned 32 bit integer        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        }; //check if user input can be converted into a number if not then continue the loop
        
        
        println!("You guessed: {}", guess); //print the value of guess

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;  //stop the game if user guess is correct
            }
        }

    }
}
