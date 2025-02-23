# The cargo fundamentals

### Bread & Butter

```
cargo new 
cargo init
```

These can take a --lib flag which is used when creating libraries. The idea here is the program no longer requires an entry point (main.rs) as we're not executing anything (or compiling to a binary executable). Running cargo init automatically creates a github repo. To nullify this behaviour, add the "--vcs none" flag. 

```
cargo build
cargo run
cargo check
```

Build, as one would guess, builds the binaries. Run both builds and executes. 
To specify the binary run 

```
cargo run --bin <bin-name>
```
Check simply checks whether what we want to build can build. Much faster and useful to run as you code (before you want to test logic). 

### Cargo.toml Basics

This contains all the relevant information cargo needs to compile the project. By default (e.g. after cargo new my_package) it will look like 

```
[package]
name = "my_package"
version = "0.1.0"
edition = "2021"

[dependencies]
```

For good practice add 

```
authors = ["Name <email-address@domain>"]
```

to anything non-trivial. 


Just specify a name + version, which should be found on whatever you're using's git, or by searching crates.io. 

Specify multiple binaries with 

```
[[bin]]
name = "mybin"
path = "src/bin.rs"
```

Add a library with 

```
[lib]
name = "mylib"
path = "src/lib.rs"
```

Manage workspaces (in large projects) with 

```
[workspace]
members = ["crates/*"]
```

### Dependencies

The line 

```
regex = "0.1.41"
```

Is implicitly getting the maximal version $v$ w/ $0.1.41 \leq v < 0.2$, i.e. 0.1.41 = ^0.1.41. 

To get the docs for your crates, just run a 

```
cargo doc --open
```

to get a HTML file with all docs from each dependency, very useful! 