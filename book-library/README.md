# Rust Library Management Example

This repository contains a simple Rust program demonstrating a **basic library management system**. It showcases how to define custom structs, implement methods, and utilize Rust features like `Option`, iterators, and the `Display` trait.

---

## Overview

In this example, we have:

- A `Book` struct that stores:
    - `title: String`
    - `author: String`
    - `year_published: u32`
    - `checked_out_by: Option<String>` (the name of the person who has checked out the book)

- A `Library` struct that stores:
    - `books: Vec<Book>` (a collection of `Book` instances)

### Key Features

1. **Adding Books**  
   You can add new `Book` instances to the `Library`.

2. **Listing Books**  
   You can list all books currently in the library. Each `Book` implements the `Display` trait for human-readable output.

3. **Finding a Book by Title**  
   You can search for a `Book` by title and receive an `Option<&mut Book>` reference if it exists.

4. **Deleting Books by Title**  
   You can remove books from the `Library` based on their title.

5. **Borrowing Books**  
   A book can be checked out with a borrower’s name, updating its `checked_out_by` field.

---
How to Run
1. **Install Rust**  
   Make sure you have the [Rust toolchain](https://www.rust-lang.org/tools/install) installed. Verify with:
   ```sh
   rustc --version
   cargo --version

2. **Clone the Repository**

   ```sh
    git clone https://github.com/MrAbdul/learningRust.git
    cd rust-practice-projects

3. **Navigate to a Project**  
   For example, to explore the Book Library System:
    ```sh
   cd book-library

4. **Build and Run**
   ```sh
   cargo run
5. **Test (If tests are included)**
   ```sh
   cargo test
