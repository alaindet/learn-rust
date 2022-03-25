# Ownership

- Ownership is at the core of Rust and one of its unique features
- It's a unique memory management mechanism different from manual allocation and from garbage collection
- Manual allocation can be difficult and error-prone
- Garbage collection is reliable and automatic, but it impacts performance (although negligibly most of the times) since the program has to periodically check unused memory and free it
- Ownership tracks references to values and wipes values at predictable points in time

## Stack and Heap
- They both are parts of memory used at runtime by any program
- The **stack** is ordered, anything inside has a fixed-length, it follows a LIFO structure and accessing is sequential (as in "you can only read adjacent memory addresses, sequentially")
- The stack is mostly used for values of known-size, mostly primitives and/or small values, function calls and anything fast
- The **heap** is mostly used for large and/or of unknown size values
- What happens is the program allocates memory for a value in the heap to some address, usually the size is equal to or of bigger size than the value, then returns the address as a pointer to the stack in order to be able to read the value later
- The heap is slower to allocate and read, but allows bigger values, frequently changing sizes and values of unknown size at compile time

## Ownership rules

- Each value in Rust has a variable called its **owner**
- One value can have one owner at a time
- When the owner goes out of scope, the memory containing the value gets wiped
- Rust has a function called `drop` which is called automatically when a value gets out of scope **naturally**, for example at the end of the scope

## Variable scope
```rust
fn main() {
    let hello = "Hello World"; // <-- hello comes into scope of this block
    // ...
} // <-- hello goes out of scope
```

## Move

```rust
let a = 5;
let b = a; // <-- This is copied in stack

let s1 = "hello";
let s2 = s1; // <-- This is copied in stack

let s3 = String::from("hello");
let s4 = s3; // <-- This is a shallow copy
println!("s3: {}, s4: {}", s3, s4); // <-- This fails as s3 is invalid now
println!("s4: {}", s4); // <-- This works
```

- `a` and `b` are two values living in the stack since they have fixed length and they are copied when assigned
- `s1` and `s2` are two values living in the stack anyway since a string literal is immutable and has fixed length
- `s3` creates a `String` type which has variable length, hence lives in the heap
- This means that `s3` is split between stack and heap like this
  - The stack contains the *length* (actual content length), the *capacity* (reserved length) and a *pointer* to the heap's address
  - The heap contains the actual string value at the pointer's address
- `s4` creates a new stack value with the pointer having the same address as `s3`, while the heap is not affected
- This means `s3` and `s4` are two different stack values pointing to the same value in the heap, so `s4` is a **shallow copy** of `s3`
- *NOTE*: A deep and a shallow copy do not make sense for stack-only values, only for stack-and-heap values
- *NOTE*: Since values can have only one variable, ownership of the string value **moves** from `s3` to `s4` so that `s3` cannot get accessed after the shallow copy
- Trying to acces `s3` after moving ownership gives error [E0382](https://doc.rust-lang.org/stable/error-index.html#E0382)
  ```
  error[E0382]: borrow of moved value: `s3`
  ```
- `a`, `b`, `s1`, `s2` and `s4` are dropped at the end of the block

## Clone

```rust
// Same as above...
let s3 = String::from("hello");
let s4 = s3.clone(); // <-- This is a deep copy now
println!("s3: {}, s4: {}", s3, s4); // <-- This now works
```

- Now `s3` and `s4` both have a stack value and heap value each, so each value in the heap has one variable assigned
- Notice that for big values the content on the heap is doubled (performance hit)
- Generally speaking, you require `.clone()` when trying to clone a non-basic value
- Basic (stack-only) values copied by default are
  - All integer numbers
  - All floating point numbers
  - Booleans
  - `char` types
  - Tuples containing only types listed here
