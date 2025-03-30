# Common Programming Concepts Notes 

Notes correspond to Ch3 in [the rust programming language](https://doc.rust-lang.org/stable/book/). 

### Variables and Mutability

Variables are immutable by default, for safety purposes, but can be made mutable by adding keyword mut. e.g. 

```
let x = 5; // immutable 
let mut x = 5; // mutable 
```

We can define some kind of super immutability with the const keyword. These are variables that cannot be made immutable down the road, i.e. will never change. e.g. 

```
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; 
// Note this is computed at compile time, const computation only legal in certain cases
```

For an immutable variable, we can achieve mutable esque behaviour by shadowing. If we redeclare a variable, the compiler will use this version until redeclared again, shadowing the first. e.g. 

```
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

will take the immutable x=5, shadow it with an immutable x = x+1 (i.e. the original var + 1), again shadow it with an immutable x = x * 2 (i.e. the original var + 1 then * 2). On print 1, this final var will be the version of x in scope, 12, and on print 2 it will be x + 1 = 6. Note we define a new scope be inserting code into curly braces. 

The key difference with mutability is that we need to assign with let when shadowing as we're creating something new. 

### Data Types

##### Scalar types 

These are types corresponding to a single value, of which 4 exist: integer, floating-point number, booleans and characters. 

---- Integers ----

Integers are specified with i32/u32, the i/u corresponds to signed/unsigned respectively, the 32 to the number of bits used to store the integer. We can have #bits in {8, 16, 32, 64, size}, where size depends on the architecture of the system you're working on (e.g. 64 bit / 32 bit architecture). 

We can define integer literals in a myriad of ways. Some examples are given below 

```{rust}
// Decimal (equals 42)
let dec = 42; 

// Hexadecimal (0xFF equals 255)
let hex = 0xFF;

// Octal (0o77 equals 63)
let oct = 0o77;

// Binary with underscore for readability (0b1010_1010 equals 170)
let bin = 0b1010_1010;

// Byte literal (b'A' equals ASCII 65, type is u8)
let byte = b'A';

// Specifying a type suffix (42u128 still equals 42)
let big: u128 = 42u128;

// Using underscores in large numbers (1_000_000 equals 1000000)
let million = 1_000_000;
```

The come with several builtin operations (e.g. addition, subtraction, remainder). See [the table](https://doc.rust-lang.org/stable/book/appendix-02-operators.html) for reference.

---- Floating-Point Numbers ---- 

These are either 32 bit or 64 bit (64 by default, works best with most modern cpus). These are representing according to the IEEE-754 standard. e.g. 

```
let x = 2.0; // f64
let y: f32 = 3.0; // f32
```

As with integers these come equipped with builtin operations. See [the table](https://doc.rust-lang.org/stable/book/appendix-02-operators.html) for reference.

--- Booleans ---

As usual, these are just true / false and use one bit (It appears it takes a whole byte but docs seem unclear, should check this). See e.g.

```
let t = true;

let f: bool = false; // with explicit type annotation
```

--- chars --- 

Primitive alphabetic type, stores a char literal (whatever is in '') with 4 bytes, corresponding to the unicode scalar value. See e,g.

```
let c = 'z';
let z: char = 'ℤ'; // with explicit type annotation (rust-analyzer gives type annotation by default)
let heart_eyed_cat = '😻';
```

Adding more than one alphabet character will cause an error, need string types to handle these.

##### Compound types

We can group multiple values into one type in two ways (primitively), with tuples and arrays. 

--- Tuples ---

Groups together a fixed (once declared immutable) number of variables of perhaps different type. Consider e.g. 

```
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup; // destructuring, lets x,y,z = first, second, third element of tup. 
println!("The value of y is: {y}"); // prints 6.4
``` 

Empty tuple () is called a unit. Expressions implicitly return this if they don't return anything else (python equivalent is probably None). 

--- Arrays --- 

Here the type is now fixed, as well as the size. Useful when we want data on the stack not the heap. e.g. 

```
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a: [i32; 5] = [3; 5]; // = [5,5,5,5,5], colon denotes repeat operation. 
```

We access array elements as you'd expect, with a[i] for i+1st element (indexing from 0 as expected). Invalid access error (e.g. let b = a[5] w/ a as above) will proceed in a runtime error (obviously, can't know before running!). This kind of memory checking, in low-level languages, is often not given (e.g. can access invalid memory, causing much deeper issues) and is a nice rust safety feature.

#### Strings

We can use the string literal, &str, to deal with strings, or the much more powerful alternative type, String, given by rust (which has a bunch of methods, properties and most importantly is mutable!). More on these later. 

### Functions 

Use fn keyword to define a function. The most important example is the main function, which is the entrypoint of most programs. Snake casing is the convention for naming here. Simple ex is given below, curly braces giving the scope of the function. As seen, we can call other functions from the main function, 

```
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

We must declare parameter types for obvious reasons. See e.g. 

```
fn main() {
    print_labeled_measurement(5, 'h'); // prints "5h"
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

##### Statements & Expressions 

Statements are instructions to be executed, not returning a value. Expressions evaluate to a resultant value. e.g. 

```
let y = 6; // statement
let x = (let y = 6); // error, expects expression gets statement
let a = {
    let x = 3; 
    x+1 // Expressions do not end in semi-colons!
}; // a = 4, interior scope ends in expression so this is well defined. 

fn five() -> i32 {
    5 // if we were to add a semi colon we'd return the unit (empty tuple) (), no good (mismatched type error)!
} // easy function w/ return value, again note we do not end with a semi colon

// These two lines are hence equivalent
let x = 5; 
let x = five();
```

### Control Flow 

##### if-else if-else logic

We want to run code depending on the truth value of a condition. This can done, as with basically other language, with if-else statements. e.g. 

```
let number = 3;

if number < 5 { // condition must be bool, so something like "if number" fails. Doesn't convert like python
    println!("condition was true"); // runs
} else {
    println!("condition was false"); // doesn't run
}
```

We call these interior scopes "arms", as seen (e.g. guessing game) with match expressions. We can chain together ifs with if, else if logic as expected. e.g. 

```    
let number = 6;

if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else if number % 2 == 0 {
    println!("number is divisible by 2");
} else {
    println!("number is not divisible by 4, 3, or 2");
}
```

The if statement is an expression, so we may use it in let statements. e.g. 

```
let condition = true;
let number = if condition { 5 } else { 6 }; // wont work if types are mismatched, variables need a single type

println!("The value of number is: {number}"); // prints 5
```

##### Loops

Rust has three keywords for loops: "loop", "while" and "for". Loop is an inf loop, the others are exactly what you expect. Break out of infinite loops (e.g. loop statements or whiles with bad conditions) with "break" keywords. continue keyword says to skip the rest of the code in the loop scope and move on to the next iteration. e.g. 

```
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};

println!("The result is {result}"); // prints 20
```

We can label loops to remove ambiguity in nested loop scenarios. e.g. 

```
let mut count = 0;
// Count up from 0 to 2
'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
        println!("remaining = {remaining}");
        if remaining == 9 {
            break; // exits inner loop
        }
        if count == 2 {
            break 'counting_up; // exits outer loop
        }
        remaining -= 1;
    }

    count += 1;
}
println!("End count = {count}");
```

While loops work as you'd expect. e.g. 

```
let mut number = 3;

while number != 0 {
    println!("{number}!"); // prints 3 then 2 then 1

    number -= 1;
}

println!("LIFTOFF!!!");
```

To iterate through elements in array, we can use for loops as you'd expect. 

```
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("the value is: {element}");
}
```

We can also do "for i in range" logic using Range, given in std library. see e.g. 

```
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
``` 

Much nicer! For loops >>>>> While loops in general, much safer.



