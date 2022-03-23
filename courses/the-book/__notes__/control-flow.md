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
  }
  ```

## Loops
