# Rust Shapes Example

This project demonstrates a simple example of defining and working with two geometric shapes in Rust: a **circle** and a **rectangle**. It calculates and prints their areas, as well as other relevant information such as the rectangle’s **perimeter** or the circle’s **circumference**.

## Features

- **Struct Definition**  
  Defines both `Rectangle` and `Circle` as separate structs with their relevant dimensions.

- **Implementation Blocks**  
  Each struct has associated methods to calculate:
    - `area` for both shapes
    - `perimeter` for rectangles
    - `circumference` for circles

- **Enums**  
  An enum `Shape` is used to unify both `Circle` and `Rectangle`, simplifying how you work with different shapes in a single function.

- **Pattern Matching**  
  Uses Rust’s `match` expression to determine the shape type at runtime and print details accordingly.

## Usage

1. **Clone or download** this repository (or copy the contents into your own Rust project).
2. Make sure you have the [Rust toolchain](https://www.rust-lang.org/tools/install) installed.
3. **Navigate** to the directory containing the `Cargo.toml` file (or create one if you're just using the example code in a single file).
4. **Build** the project:
   ```sh
   cargo build
   cargo run

## Explanation of Key Parts

-   **`#[derive(Debug)]`**  
    Allows printing struct instances with the `{:?}` format specifier.

-   **Calculation Methods**

    -   `area`: Computes the area for both shapes.
    -   `perimeter`: Computes the perimeter for rectangles.
    -   `circumference`: Computes the circumference for circles.
-   **`enum Shape`**  
    Demonstrates how a single enum can group different struct types for convenient handling.

-   **Pattern Matching**  
    Distinguishes between `Circle` or `Rectangle` and executes the relevant calculations and prints.


Feel free to extend this example by adding more shapes or functionalities, such as volume calculations for 3D shapes or additional transformations (rotations, translations, etc.).