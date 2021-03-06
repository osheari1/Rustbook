# Cargo
## Creating a new project
Default mode for 'build' is debug.
* cargo build {--release}
* cargo run

## Dependencies
Add dependencies that exist on crates.io
* crateName = "version"

This will  build with the most recent commit in master
* rand = { git "https://github.com/rust-lang-nursery/rand.git" }

You should generally only build using the above method, because cargo will automatically track which commit the package was built with in Cargo.lock.
Once ready to update the package:
* cargo update
* cargo update -p rand

You can specify which commit as well
* rand = { git "https://github.com/rust-lang-nursery/rand.git", rev = "9f35b8e" }

### Sub-crates
Path dependencies are sub-creates that live in one repository.

Inside original project run
``` bash
cargo new sub_create
```

open hello_world/Cargo.toml and add
``` toml
[dependencies]
sub_crate = { path = "sub_crate" }
```

### Testing a Bugfix
When working with a crate from crates.io and find a bug in the package, you can fix the bug locally.
Imagine the following scenario, where uuid has a bug.
```toml
[package]
name = "my-lib"
version = "0.1.0"
authors = ["..."]

[dependencies]
uuid = "1.0"
```

Next clone the uuid repo from github into some directory.

Edit the manifest of my-lib with the following
```
[patch.creates-io]
uuid = { path = "../path/to/uuid"}
```
Then run
```cargo build
```

### Unpublished Minor Version
Use this when you have locally implemented a new feature, tested it, and created a pull request. Do this when the repository has not been updated yet.

```toml
[package]
name = "my-lib"
version = "0.1.0"
authors = ["..."]

[dependencies]
uuid = "1.0.1"


[patch.creates-io]
uuid = { git = "https://github.com/rust-lang-nursery/uuid"}

```

### Overriding repository URL
If dependency in question is not on crates.io

```
[patch."https://github.com/your/repository"]
my-lib = { path = "../my-lib/path"}
```

### Publish a Breaking Change
When publishing a major release that breaks code....
```
[dependencies]
uuid = "2.0"

[patch.crates-io]
uuid = { git = "https://github.com/rust-lang.nursery/uuid", branch = "2.0.0"}
```



## Project Structure
* Default lib file = src/lib/rs
* Default executable file = src/main.rs
* Additional executable files can be placed in src/bin/*.rs
* Integration tests go in teh tests directory
* Unit tests go in each file they're testing
* Examples go in the examples directory
* Benchmarks go in the benches directory

## Tests
Run tests with
* cargo test
This will also compile any examples and test them as well.



