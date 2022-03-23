fn main() {
    let immutable_var = 1;
    let mut mutable_var = 2;
    mutable_var = 3;
    let _intentionally_not_used_var = 3; // Note the underscore
    println!("immut: {}, mut: {}", immutable_var, mutable_var); // immut: 1, mut: 3

    // Shadowing (preserves immutability, gives error on "simple" assignments)
    let x = 1;
    let x = x + 1;
    let x = x * 2;
    // x = 2; // This throws an error
    println!("X: {}", x); // X: 4

    // Mutable alternative
    let mut y = 1;
    y = y + 1;
    y = y * 2;
    println!("Y: {}", x); // Y: 4
}
