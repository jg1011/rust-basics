use std::io; // Bring input-output library into scope from standard library
use rand::Rng; // Bring Rng trait into scope from rand crate
use std::cmp::Ordering; // Bring ordering type into scope, enum in {Less, Greater, Equal}

fn main() {
    println!("Lets play a game of what is my program thinking");

    // Generate secret number local to the current thread of execution (via thread_rng)
    let secret_number = rand::thread_rng().gen_range(1..=100); // gen_range from Rng trait
    // The ..= is a range INCLUSIVE literal, doing 1..100 would gen in {1,2,...,99}

    loop { // inf game loop, ctrl c to interrupt
        println!("Guess an integer from 1-100"); 

        let mut guess = String::new(); // String object is growable, append with push. 
    
        io::stdin() // Read console input
            .read_line(&mut guess) // Reads line into guess via mutable pointer machinery
            .expect("failed to read line");
        
        let guess: u32 = match guess.trim().parse() { // Shadow variable into u32 type
            Ok(num) => num,
            Err(_) => continue // just ignore bad guesses, _ is a catchall so matches to all errors
        }; 
        // .trim() kills whitespace, .parse() attempts to convert to desired type, returning result enum
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number){ // cmp method well defined on u32 types
            Ordering::Less => println!("More"),
            Ordering::Greater => println!("Less"),
            Ordering::Equal => {
                println!("Winner winner chicken dinner!");
                break; // Escape the insatiable infinite loop
            }
        };

    }
}
