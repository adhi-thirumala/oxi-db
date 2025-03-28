# Oxi-DB
## Who
Adhi Thirumala (netid: adhit2)
## What
 - CS128 Honors Final Project at UIUC
 - File Based Relational Database and Rust Library for interfacing
 - Written in Blazing Fast Rust
 - Automatically detects database and table schema while auto-converting to Rust Types
## Why
 - Not many production-grade file-based databases written in Rust (probably because SQLite is really, really good)
 - I want to learn data structures and database theory
 - I want to get better at Rust
## When
 - My freshman year of college
## Where
 - Champaign-Urbana (or wherever you, the hypothetical new contributer are)
## Challenges
  - Concurrency and async is hard for DB operations
  - Creating a way to make the DB types map exactly to Rust Types
  - making a crate
## How
### Checkpoint 1
Choose the datastructure to implement the database upon (probably b-tree) and implement it in rust + test cases
 - Implement a basic B-Tree in Rust
 - Test cases for insertion, deletion, and traversal
 - Ensure it can handle basic operations like searching for a key
 - Make a list of all the operations that we want to do on the DB overall + types we will support, probably want something like a sum type like json
 - figure out how to save the file
### Checkpoint 2
 - Make a crate to interface with the database
 - Create a basic API for the crate that allows users to create a database, create tables, insert records, and query records
 - Implement the API in Rust, ensuring that it can handle basic operations like creating a database, creating a table, inserting records, and querying records
 - maybe do some async things to allow for concurrent access to the database and concurrent writes
  
### Running
 - do `cargo run`
### Testing
 - do `cargo test`
### Installation from [crates.io](https://www.crates.io)
  - TBD




## Contribution Guidelines
### Commit Message Convention
 
This website follows [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/). Make sure to use them! If you're lazy, check out [git-commit-ai](https://www.github.com/adhi-thirumala/git-commit-ai) and just use that.
