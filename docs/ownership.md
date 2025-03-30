# Ownership 

Unique to rust, allows for memory safety without garbage collection. 

### Stacks & Heaps

Stacks are first in last out structures. Heaps add data in whatever empty space is available, supplying a pointer to the added data. Heaps are slower for both write/read, requires searching. When we run a function, the values passed into the function are put on the stack. After use, they are popped. 

The idea of ownership is to solve the problem of keeping track of what is on the heap, reducing duplicate data on the heap and cleaning up unused data on the heap. Ownership => we (except in rare cases) need not think about the heap. 

### Ownership Rules

- Each value has an owner
- There can only be one owner at a fixed time
- Owner leaves scope => value dropped.

### An example with the String type

The &str literal is generally a bad way of handling strings (e.g. immutable). To create a better string object, we have the builtin String type in rust (seen briefly in common-concepts). We need to allocate an unknown (at runtime) amount of memory to the heap to deal with this. I.E. we need 

- Memory to be requested from memory allocator at runtime
- Memory to be returned to allocator when done with string

We tell the memory allocator how much memory we need when we declare a string with 

```
let s = String::from("string with some funny name or hello world or whatever you feel like");
```

The second is more complicated. In languages with garbage collection this is done under the hood for us. For languages without, we usually have to do it ourselves. Rust takes a third route, instead returning memory once the variable leaves scope. This is done with the "drop" function, a builtin rust method that runs whenever a closing curly brace is used. 

This is profound for a few reasons. The first can be seen with the variable copying example. Consider 

```
let x = 5;
let y = x;
```

This works as we expect. The value 5 is bound to x, then x is copied and bound to a new variable y. 

Now consider 

```
let s1 = String::from("hello");
let s2 = s1;
```

This does not work the same. A string is composed of a pointer, length and a capacity. We need not copy everything this time, reducing the total amount of memory on the heap. Consider the following diagram, which shows our copy doesn't need to add anything to the heap to work. This results in a factor of 2 space complexity gain.

![figure 1](figs/fig1.svg)

Now suppose drop is ran. We would end up trying to drop the same thing twice, resulting in a double free error. This is no good. Instead, after copying, rust considers the first variable to be invalid. Indeed, the following code doesn't compile. 

```
let s1 = String::from("hello");
let s2 = s1;
println!("{s1}, world!");
```

As we're not really copying anymore, we refer to this kind of thing as a move. 

This logic means we never really create deep copies (i.e. rewriting the same data twice). As such, copying is inexpensive and we may do it automatically if necessary. 

When we rewrite a variable, we immediately drop the old version and add the new to the stack, updating pointers accordingly. See the following diagram + code. 

```
let mut s = String::from("hello");
s = String::from("ahoy");
println!("{s}, world!");
```

![figure 2](figs/fig2.svg)

If, for whatever reason, we do want a deep copy, we can just use the builtin clone method. 

--- Why is this not the case for integers?

The amount of memory required to store is known at compile time and hence can be stored entirely on the stack. Hence, copies are also quick to make and we "may as well" make a deep copy when copying.

For an example with functions, which behave similarly to variables, see the following code. 

```
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // because i32 implements the Copy trait,
                                    // x does NOT move into the function,
    println!("{}", x);              // so it's okay to use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
```

The return values also transfer scope. Consider the following example. 

```
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

Managing ownership seems to be more effort than its worth. Thank god for references...

### References 

These are just pointers, rustaceans have their own name for them.

Using our above notion of ownership, things become quite tedious. Suppose we want a tuple composed of a string and 
its length. We'd need to do something like...

```
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

The key issue is we need to return our parameter, as well as the function output, as a tuple. This should be quicker. Thankfully, we can just point to the string rather than changing its owner, so that both items need not be returned. Consider

```
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

This works analogously. The ampersand corresponds to a pointer to the variable. The variable s does leave scope when the function is complete, but not the string s1. We call the action of referencing another variable "borrowing".

As with other variables, references (i.e. pointers) are immutable by default. Thus we cannot change the value of the underlying variable our reference points to (i.e. borrows from). Indeed, the following code will not compile.

```
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

We can modify it to work by using mutable pointers. Indeed, the following fix does compile. 

```
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

We cannot have more than one mutable pointer to a variable being in scope at any one moment. This is to avoid data racing issues at compile time. Data races occur when 

- $\geq 2$ pointers exist to the data at the same moment
- $\geq 1$ pointers write to the data
- There is no mechanism synchronising access to the data.

Data races cause undefined behaviour, and are difficult to debug. Rust stops this problem in its tracks. To get around this, further mutable pointers must be defined in their own scope. Then a synchronisation process is given and we're happy. This logic also applies to having a mutable and immutable reference to the same variable in the same scope. 

The compiler knows when the scope ends, so we need not always use curly braces to define the scope. If the variables are no longer used past a certain point, the scope for them is over. Indeed, the following code compiles. 

```
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{r1} and {r2}");
// variables r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{r3}");
```

If we reference to a nonexistent value, we have a dangling reference. These are caught at compile time, and result in a lifetime error (discussed later).

### Slices

For strings we can point to substrings, i.e. slices, in the usual kinda way. Uninteresting really and seen in other languages. We introduce these by solving the "find first word problem" for strings and motivate further by considering the "find k'th word problem". Consider the following solution 

```
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // Convert string to bytes

    for (i, &item) in bytes.iter().enumerate() { // Enumerate through characters
        if item == b' ' { // If character is space
            return &s[0..i];
        }
    }

    &s[..] // If no space detected, the whole string is one word!
}
```

This logic generalises to e.g. arrays, but you know what slicing is so I won't elaborate. See sec4.3 for details. 




