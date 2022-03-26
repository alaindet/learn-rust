# Structs (structures)

```rust
struct User { // <-- This is a struct definition
  username: String, // username is a **field**, String is the field's type
  email: String,
  is_active: bool,
}

fn main () {
    // A mutable instance
    let mut user1 = User {
      username: String::from("user1"),
      email: String::from("user1@example.com"),
      is_active: true,
    };

    // Mutate username
    user1.username = String::from("user1_edited");

    // An immutable instance
    let user2 = User {
      username: String::from("user2"),
      email: String::from("user2@example.com"),
      ..user1 // <-- This means "copy all not-explicitly-assigned values from user1"
    };

    println!("{}: <{}>, ACTIVE? {}", user1.username, user1.email, user1.is_active);
    // user1_edited: <user1@example.com>, ACTIVE? true

    println!("{}: <{}>, ACTIVE? {}", user2.username, user2.email, user2.is_active);
    // user2: <user2@example.com>, ACTIVE? true
}
```

## Tuple structs
- This is a particular kind of struct that looks like a tuple
- Fields do not have names, only types
- It's convienient to distinguish two semantically different tuples with the same underlying structure

Example
```rust
struct Color(i32, i32, i32)
struct Point(i32, i32, i32)

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

In the example, `black` and `origin` have exactly the same structure and even the same values, but they are of different types and have two completely different meanings
