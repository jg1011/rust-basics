# Structs 

## Defining and Instantiating Structs

Define a struct by supplying the names and types of the data stored in the struct, e.g. 

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

We can instantiate this struct by doing 

```
let user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
};
```

Access values from struct with dot notation. We may only change values if our instance is mutable, e.g. 

```
let mut user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
}; 

user1.email = String::from("newemail@example.com");
```

Lets say we wanted to write a function to build a new user. We default the sign_in_count value to 1 and the active 
value to true. We don't want to write the username/email variable twice, so we use the field init shorthand seen below. 

```
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // rather than username: username
        email, // rather than email: email
        sign_in_count: 1,
    }
}
```

Note this only works due to the shared naming. 

We can update structs into new structs as follows 

```
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

This will copy all the values that aren't email from user1 into the new user2 instance but update the email. 

If we don't care for value names, we can define tuple structs. Consider 

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

We can also define empty structs, called unit-structs, which behave like the unit type (empty tuple). This is useful when we want to implement a trait on a type but don't have any data we wish to store. 

Printing a struct is awkward, we wont have this trait implemented so we need to tell the compiler to try its best by using :? (or :#? for pretty print) as follows

```
#[derive(Debug)] // Needed as :? needs debugging and this isn't implemented by default for our struct
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
}
```

Methods are done with so called implementations. See the following for syntax 


```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

We can add several functions in the impl block in the obvious way. We could also add multiple impl blocks. 