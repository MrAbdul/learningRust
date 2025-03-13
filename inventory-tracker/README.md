# Inventory Management in Rust

This project is a simple command-line application demonstrating how to manage an in-memory inventory using Rust. It covers some fundamental Rust concepts including:

-   Defining custom `struct`s
-   Using `impl` blocks to implement methods
-   Managing ownership and borrowing
-   Utilizing the struct update syntax for `Copy` types (`u32`, `f32`)


## What You’ll Learn

-   **Structs and Methods**  
    You’ll see how to define structs with data fields and implement methods on them.
-   **Ownership and Borrowing**  
    Notice how references (`&mut self`) allow modifying the inventory.
-   **Struct Update Syntax**  
    Learn how to create a new instance of a struct by copying some fields from another instance.  
    This syntax is particularly handy when the fields being reused implement the `Copy` trait (like `u32` and `f32`).
-   **Vector Management**  
    You’ll see how to push new items onto a `Vec<Item>` and iterate over it.

## Possible Extensions

-   **Discount Field**  
    Add an optional discount field (`f32`) to `Item` and incorporate it into the total value calculation.
-   **Serialization**  
    Use Serde to serialize/deserialize the inventory to JSON, TOML, or YAML.
-   **File Persistence**  
    Save the inventory to a file on disk and load it back on the next run.

## License

This project is provided as-is for learning purposes and is not under a specific license. Feel free to adapt, modify, or incorporate the ideas into your own projects.

* * *

Enjoy exploring Rust with this small inventory manager! If you find any issues or have ideas for improvements, feel free to open an issue or submit a pull request. Have fun coding!

