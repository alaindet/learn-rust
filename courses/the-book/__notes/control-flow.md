# Control Flow

## If
- Controls the flow with conditions
- Blocks following if, else and else if are called **arms**
- Conditions following if and else if **MUST** be booleans
- An error is thrown if not boolean is used in a condition

```rust
fn main() {
  let x = 42;

  if x == 42 {
    println!("Yep");
  } else {
    println!("Nope");
  }
}
```

- An `if` is an expression, meaning you could do something (rather silly) like
  ```rust
  let should_i = true;
  let n = if should_i {
      10
  } else {
      20
  }; // <-- Mind this
  // n = 10
  ```

- When using an if as an expression, values evaluated from any arm **must be of the same type**
  ```rust
  let sure = true;
  let a = if sure {
      5
  } else {
      6 // GOOD
      6.0 // BAD
  }; // <-- Mind this!
  ```

## Loops
- Rust has three types of loop: `loop`, `while` and `for`

### `loop`
- Loops infinetely until you explicitly `break`
  ```rust
  loop {
    // Repeated code...
    if some_exit_condition {
      break; // <-- You exit the loop with this
    }
  }
  ```
- Common use cases are for checking threads, or retrying on failed user input
- Being an expression, it can be **used as a value**
  ```rust
  let mut counter = 0;
  let result = loop {
      counter += 1;
      if counter == 10 {
          break counter * 2; // <-- This assigns 20 to result
      }
  };
  println!("The result is {}", result); // The result is 20
  ```

### `while`
- It can derived from `loop`, `if`, `else` and `break`, but it's convenient to use `while`
  ```rust
  let mut n = 3;
  while n != 0 {
      println!("{}!", n);
      n = n - 1;
  }
  println!("Liftoff!");

  // Equivalent
  let m = 3;
  loop {
      if m != 0 {
          println!("{}!", n);
          n = n - 1;
      } else {
          break;
      }
  }
  println!("Liftoff!");
  ```

### `for`
```rust
let a = [10, 20, 30, 40, 50];

for element in a.iter() {
    println!("the value is: {}", element);
}

// Equivalent
let mut i = 0;
let a_length = a.len();
while i < a_length {
    println!("the value is: {}", a[i]);
    i = i + 1;
}
```
