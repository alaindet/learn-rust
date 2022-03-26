# Ownership and slices

```rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
let up_to_three = &s[..3];
let from_two_on = &s[2..];
let all = &s[..];
// let all = &s[0..s.len()]; // Equivalent

println!("s: {}", s); // s: hello world
println!("hello: {}", hello); // hello: hello
println!("world: {}", world); // world: world
println!("up_to_three: {}", up_to_three); // up_to_three: hel
println!("from_two_on: {}", from_two_on); // from_two_on: llo world
println!("all: {}", all); // all: hello world
```

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let s = String::from("hello world");
    let s_literal = "hello world";
    let f1 = first_word(&s[..]);
    let f2 = first_word(&s_literal[..]);
    let f3 = first_word(s_literal);
    println!("f1: {}", f1); // f1: hello
    println!("f2: {}", f2); // f2: hello
    println!("f3: {}", f3); // f3: hello
}
```

- **NOTE**: in Rust, `String` is a dynamic heap type, while `str` an immutable sequence of UTF-8 bytes
- **NOTE**: `str` is a `String`, but `String` is not `str`!
- `str` is a subcategory of `String`
