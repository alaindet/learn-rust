fn if_example() {
    let x = 42;

    if x == 42 {
        println!("Yep");
    } else {
        println!("Nope");
    }

    // Use if as value
    let should = true;
    let a = if should { 10 } else { 20 }; // <-- Mind this

    let b = if should {
        5
    } else {
        6 // Good
          // 6.0 // Bad
    };

    println!("a: {}, b: {}", a, b); // a: 10, b: 5
}

fn loop_example() {
    let mut i = 0;
    loop {
        if i < 5 {
            i = i + 1;
        } else {
            break; // <-- Exit the loop!
        }
    }
    println!("i: {}", i); // i: 5

    // Use loop as a value
    let mut j = 0;
    let result = loop {
        j += 1;
        if j == 10 {
            break j * 2; // <-- This assigns 20 to result
        }
    };
    println!("result: {}", result); // result: 20
}

fn while_example() {
    println!("# While");
    let mut n = 3;
    while n != 0 {
        println!("{}...", n);
        n = n - 1;
    }
    println!("Liftoff!");

    // Equivalent
    println!("# Loop");
    let mut m = 3;
    loop {
        if m != 0 {
            println!("{}...", m);
            m = m - 1;
        } else {
            break;
        }
    }
    println!("Liftoff!");

    // Equivalent
    println!("# For");
    for t in (1..4).rev() {
        println!("{}...", t);
    }
    println!("Liftoff!");
}

fn for_example() {
    let a = [10, 20, 30, 40, 50];

    println!("# For");
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Equivalent
    println!("# While");
    let mut i = 0;
    let a_length = a.len();
    while i < a_length {
        println!("the value is: {}", a[i]);
        i = i + 1;
    }
}

fn main() {
    if_example();
    loop_example();
    while_example();
    for_example();
}
