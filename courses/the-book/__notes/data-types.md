# Data types
- Data types describe the "category" of a variable (string, number etc)
- Data types can be
  - **scalar** representing a single value; Rust has four scalar types:
    - integers
    - floating-point numbers
    - booleans
    - characters
  - **compound** representing a related group of values; Rust has two primitive compound types:
    - tuples
    - arrays

## Scalar: Integer numbers

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8 bit   | i8     | u8       |
| 16 bit  | i16    | u16      |
| 32 bit  | i32    | u32      |
| 64 bit  | i64    | u64      |
| 128 bit | i128   | u128     |
| arch    | isize  | usize    |

- The default integer (inferred) is `i32` since it's faster than `i64` even on 64-bit computers
- `isize` and `usize` are signed and unsigned integers based on the computer architecture, meaning they can be equivalent to `i32` and `u32` or most commonly `i64` and `u64` these days
- Visual separator `_` can be used to make big numbers more readable, ex.:
  ```rust
  const BIG_NUMBER = 42_000_000
  ```

### Overflow
- In debug mode, integer overflow causes a panic error
- In release mode, integer overflow does not crash the program and wraps instead

## Scalar: Floating-point numbers
- Floating-point number types are `f32` and `f64` for 32-bit and 64-bit precision
- The default inferred floating-point type is `f64` since it allows for greater numbers and decimal precision than `f32`
- Example
  ```rust
  let x = 2.0; // f64 (inferred by default)
  let y: f32 = 3.0; // f32
  ```

## Scalar: Booleans
- Simple as that
```rust
let yep = true; // Inferred type
let nope: bool = false; // Annotated type
```

## Scalar: Characters
- Strings are wrapped in double qoutes, characters are wrapped in single quotes
- The primitive `char` type is 32-bit long representing a Unicode Scalar Value
```rust
let some_char = 'z';
let a_string = "Hello World";
let annotated_char: char = 'a';
```

## Compound: Tuples
- A tuple is a fixed-length collection of one or more values of possibly different types
  ```rust
  let my_tuple: (i32, f64, u8) = (500, 6.4, 1); // Annotated
  let my_tuple2 = ('a', 123, 4.56); // Inferred
  ```
- Accessing values from a tuple is possible with **destructuring** via pattern matching, or via direct access using a period
  ```rust
  let person = ("John", "Doe");

  // Via destructuring
  let (first_name, last_name) = person;
  println!("First name: {}, Last name: {}", first_name, last_name);
  // First name: John, Last name: Doe

  // Via period
  let first_name = person.0;
  let last_name = person.1;
  println!("First name: {}, Last name: {}", first_name, last_name);
  ```

## Compound: Arrays
- Arrays are very different from other languages as they're even strictier than tuples
- An array has fixed-length and elements **MUST** be all of the same type
  ```rust
  let odds = [1, 3, 5, 7, 9]; // These are all 32-bit integers
  let even: [i32; 5] = [0, 2, 4, 6, 8]; // Annotated
  ```
- There is another type called **vector** which can have a variable length
- You can initilzize an array with the same value
  ```rust
  let a = [3; 5];
  // let a = [3, 3, 3, 3, 3];
  ````
- Accessing elements
  ```rust
  let a = [1, 2, 3, 4, 5];
  let first = a[0];
  let nope = a[9]; // This throws an error at RUN TIME not COMPILE TIME
  ```
