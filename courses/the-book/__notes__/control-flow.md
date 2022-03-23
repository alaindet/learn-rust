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

## Loops
