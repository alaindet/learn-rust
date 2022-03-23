# Functions

- Functions are defined as in any other language, but prefixed with `fn`
- Function names must follow the snake_case
- Functions can use other functions declared later, as long as they are declared somewhere
  ```rust
  fn main() {
      hello_world();
  }

  fn hello_world() {
    println!("Hello World");
  }
  ```
- Function **parameters** are variables declared in the **signature** of the function
- Functions parameters become function **arguments** only when assigned a value during execution
- Parameters **MUST** have a type annotation
  ```rust
  fn main() {
      sum(1, 2); // Prints "Sum is: 3"
  }

  fn sum(a: i32, b: i32) {
      println!("Sum is: {}", a + b)
  }
  ```

## Expressions and statements
- An **expression** is a value that can be explicit or the result of some operations (ex.: 3 or 2+1)
- Expressions include function calls, scope declarations, macro calls etc
- A **statement** is an instruction to execute that can contain expressions, does not return a value or whose value is not used
- Functions can have multiple statements, but only one optional expression
- **NOTE**: Assignment is a pure statement, meaning no value is returned
  - In other languages, assignments return the assigned value, Rust doesn't
- An expression **MUST NOT** end with a semicolon, otherwise it becomes a statement
- A scope ending in an expression absorbs the value of the expression
  ```rust
  fn main() {
      let x = 5;
      let y = {
          let x = 3;
          x + 1 // <-- This is an expression! y takes this value
          // x + 1; // <-- This is a statemetn!
      };
      println!("Y: {}", y);
  }
  ```

## Return values
- Return values from functions must be typed with a `->` symbol followed by a type name
- Return values are either returned "early" via the `return` keyword or implicitly assumed if the function's last line is an expression
- NOTE: Return of implicit expression value is very specific to Rust

```rust
fn five() -> i32 {
    5
    // 5; // <-- This does not work!
}

fn five_again() -> i32 {
    return 5;
}

fn main() {
    let a = five();
    let b = five_again();
    let c = a + b;
    println!("c: {}", c); // c: 10
}
```
