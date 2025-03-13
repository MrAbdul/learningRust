Rust Practice Projects
Welcome to my collection of small Rust projects! This repository holds various exercises and mini-projects designed to help me (Abdul) practice and solidify my understanding of Rust concepts—especially those around ownership, borrowing, structs, and methods.

Table of Contents
Overview
Projects
1. Book Library System
2. Shape Calculator
3. Inventory Tracker
4. User Profile Manager
5. Team Roster
Project Structure
Setup and Usage
Contributing
License
Overview
These projects are part of my Rust learning journey. Each directory corresponds to a small, self-contained application. They help me practice key Rust concepts, including:

Structs
Methods (with &self vs. &mut self)
Ownership & Borrowing
Basic I/O and error handling
Pattern matching and options
Feel free to explore the projects in any order. Some are simpler than others, but all emphasize struct usage and fundamental Rust knowledge.

Projects
1. Book Library System
Concepts: Structs, methods, ownership, references
Description:
A small program that models a basic library system.

Book struct with fields like title, author, year_published.
Library struct containing a Vec<Book>, plus methods to add, find, or remove books.
Demonstrates how to handle Option<&Book> and Option<&mut Book>.
Key Files:

src/main.rs (or wherever you place your main module)
2. Shape Calculator
Concepts: Structs, traits, methods
Description:
Calculates areas and perimeters/circumferences for shapes.

Rectangle and Circle structs, each with their own fields.
Methods to compute area, perimeter, and circumference.
(Optional) An enum Shape to hold different shape variants.
Key Files:

src/main.rs
3. Inventory Tracker
Concepts: Struct update syntax, mutable references, vector iteration
Description:
A program that tracks items in an inventory.

Item struct with fields like name, quantity, price.
Inventory struct holding a Vec<Item>.
Methods to add, update, and calculate total value of all items.
Key Files:

src/main.rs
4. User Profile Manager
Concepts: Option types, pattern matching, method design
Description:
Stores user profiles in memory, some with optional fields (like age or email).

User struct with optional fields.
Methods to update fields, check if the profile is complete, and display user data gracefully.
Key Files:

src/main.rs
5. Team Roster
Concepts: Nested structs, summation, references
Description:
Manages a team roster with players and keeps track of total points.

Player struct with fields like name, jersey_number, points_scored.
Team struct holding a Vec<Player>.
Methods for adding players, summing total points, or finding the highest scorer.
Key Files:

src/main.rs
Project Structure
Each project is organized as a separate Cargo package. That means this repo might look like this:

css
Copy
Edit
rust-practice-projects/
├── book-library/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── shape-calculator/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── inventory-tracker/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── user-profile-manager/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── team-roster/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── README.md
Alternatively, you could store them all under a single top-level Cargo.toml file with multiple binary targets. For simplicity, I prefer each in its own folder.

Setup and Usage
Install Rust
Make sure you have the Rust toolchain installed. Verify with:
sh
Copy
Edit
rustc --version
cargo --version
Clone the Repository
sh
Copy
Edit
git clone https://github.com/YourUsername/rust-practice-projects.git
cd rust-practice-projects
Navigate to a Project
For example, to explore the Book Library System:
sh
Copy
Edit
cd book-library
Build and Run
sh
Copy
Edit
cargo run
Test (If tests are included)
sh
Copy
Edit
cargo test
Repeat the above steps for each project directory. Code away and have fun!

Contributing
I’m using these projects for personal learning, but if you have suggestions, feel free to open an issue or submit a pull request. Constructive criticism and new ideas are always welcome!

License
This repository is distributed under the MIT License. Feel free to use these exercises as a reference or modify them for your own learning.

Happy Rusting!
Thanks for stopping by! If you see any potential improvements or want to suggest new practice ideas, don’t hesitate to reach out or open an issue.

Keep practicing and enjoy the journey!
