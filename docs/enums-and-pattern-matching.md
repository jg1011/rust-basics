# Enums and Pattern Matching 

We motivate and explore the syntax of (defining) enums with the IP address example. There are two types of IP address in use, V4 and V6. We may define an IpAddr structure as follows, along with two examples. 

```
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
```

So far enums seem to need not exist. Well, consider the following simplification. 

```
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

Other than code simplicity, another advantage is allowing different variants to have different types. Consider 

```
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

#### Option Enum 

The option enum is a standard library enum which can either be something or nothing. This is an alternative to the classic null value approach. 

Tony Hoare, the inventor of null, said this about them. 

"I call it my billion-dollar mistake. At that time, I was designing the first comprehensive type system for references in an object-oriented language. My goal was to ensure that all use of references should be absolutely safe, with checking performed automatically by the compiler. But I couldn’t resist the temptation to put in a null reference, simply because it was so easy to implement. This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years."

The option enum instead does 

```
enum Option<T> { // <T> is a generic type param, so can take any type 
    None,
    Some(T),
}
```

This is included in the prelude, so no need to bring it into scope. 

Consider the following example: 

```
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

This won't compile, because addition isn't defined for Option\<i8>. We would need to convert y to an i8 to make this work. This will catch most errors with none in their tracks, making Tony Hoare happy :)

#### Match control flow construct 

Say we have an enum for the different kinds of coin (US example) and we want a function to return the value of a given type of coin. This is where match statements thrive. 

```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

We can also insert all kinds of logic based off of the match using curly braces. Consider 

```
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Suppose quarters have different designs for each US state. We can stack enums to handle logic of this kind as follows: 

```
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```

Returning to the option example, we can extract our type T from option\<T> using a match statement. Observe

```
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

NOTE: matches must be exhaustive, or the code won't compile. 

What if we want to match only a few of the pissble cases on a variable, e.g. doing something special on certain dice rolls. Consider

```
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```

If we don't want to use the other value, we can use the catchall "_", consider 

```
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
```

Or if we want absolutely nothing to happy we can use the unit 

```
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

#### if let and let else 

Consider the following snippet. 

```
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),
}
```

We can sharpen this up with the if let logic. Consider 

```
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}
```

This behaves the same as our match. If there is something in max, do the print, otherwise do nothing. There is a natural analogue for let else, see end of sec6.3 rust book.