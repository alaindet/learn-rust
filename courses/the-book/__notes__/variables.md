# Variables

- Variables in Rust are immutable by default
- In order to mutate a variable, you have to explicitly declare it with `mut`
  - Example
  ```rust
  let a = 123;
  let mut b = 456;
  b = 123; // Good
  a = 456; // Bad
  ```
- Unused variables generate warnings
- An intentionally unused variable must start with undescore (ex.: `_nope`)

## Constants
- Constants differ from immutable variables
- You can declare them in any scope and they are bound to that scope
- You cannot use `mut`
- You declare constants with `const` and not `let`
- You must declare type annotations with constants, no type inference is allowed
- You can only use simple expressions since they are evaluated at compile-time
- You must use SCREAMING_SNAKE_CASE casing

```rust
let some_variable = 1;
const THE_ANSWER: u32 = 42;
```

## Shadowing
- Re-declaring a variable with the same name is possible in Rust
- The variable re-declared **shadows** the previous one

## Shadowing vs `mut`
Shadowing
  - It's best used to change a variable occasionally leave it immutable after that
  - It makes clear that change is limited but variable is otherwise immutable
  - It allows to reuse a name (ex.: same name for user input and parsed value)
  - It allows to change the variable type (ex.: string to integer)
`mut`
  - It's best used for a value that updates frequently
  - The variable can always be changed with an assignment
  - The type cannot change
