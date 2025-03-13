## 4\. Simple “User Profile” Manager

**Concepts Practiced**:

-   Using `Option<T>` in structs
-   Creating methods that return references to fields or handle missing values
-   Pattern matching on struct fields

**Description**:  
Create a `User` struct that has:

-   `username: String`
-   `age: Option<u8>` (some users might not provide an age)
-   `email: Option<String>` (some might prefer not to provide an email)
-   `active: bool`

**Tasks**:

1.  Write methods to set or unset the optional fields (like `email` or `age`).
2.  Write a method `is_complete_profile` that returns `true` only if `age` and `email` are set.
3.  Write a function that prints out user details. For the optional fields, display “Not provided” if they’re `None`.

**Hints**:

-   Use `Option::is_some()` or pattern matching to handle the optional fields.
-   Think about whether your methods take `&mut self` or consume the `User` altogether.

**Extensions**:

-   Add validation logic (e.g., email must contain `@`)—though that’s more of a string parsing exercise.
-   For a real challenge, create a small CLI program that allows adding and viewing user profiles via the command line.