# Guessing Game Notes

These correspond to chapter 2 in [the rust programming language](https://doc.rust-lang.org/stable/book/). As such, 
everything will be analysed in great detail as we are assumed to know nothing at this point. 

### Goal 

Program a simple 1-100 guessing game. 

1. Program generates an integer from 1-100 
2. User guesses an integer from 1-100
3. Program checks if correct
4. If correct player wins, done
5. Else program says whether guess was too low/high
6. Player guesses again 
7. Repeat

### Processing a guess

##### Code

```{rust}
use std::io; 

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

##### Remarks 

1. Input/Output functionality is not in the prelude (a set of items automatically brought into scope). See the list of what is in the prelude [here](https://doc.rust-lang.org/stable/std/prelude/index.html). See what is in the std library [here](https://doc.rust-lang.org/std/).
2. The mut keyword indicates mutability of a variable, by default variables are immutable. 
3. String::new() is a function associated with the String type (in the std library, utf-8 encoded) which returns an instance of String. 
4. io::stdin() calls the stdin function from the io module (from the std library). We could use this without bringing io into scope by replacing it with std::io::stdin(). Returns an instance of io::Stdin (resp. std::io:Stdin). 
5. read_line() is a method on io::Stdin which, as you'd guess, reads lines. The argument tells us where to store this input, must be mutable as we're going to be changing the argument. The ampersand & means we're simply referencing guess, which allows us to use it without copying it into memory again. Nice and fast :)
6. .read_line() returns a Result type, either Ok or Err. Adding a .expect("err message") will either return what we want if all goes smoothly and will flag an error (program crashes with err message) if something went wrong. This is an example of an enumeration (enum shorthand often used) which is a variable that can be multiple states. 
7. The empty curly brace {} in the final print is a placeholder, which is filled by the additional argument we give println!. We could alternatively just do {guess} and drop the additional argument. 

### Generating a secret number

Not documenting here anymore, just read the code. Plenty more interesting stuff to document later, will just end up repeating myself and thus this makes for a poor reference. Comments should explain stuff well enough.



