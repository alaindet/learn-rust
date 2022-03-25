/*
 | ----------------------------------------------------------------------------
 |   EXAMPLE 1
 | ----------------------------------------------------------------------------
 */
fn example1() {
    let s = String::from("hello");
    take_ownership(s);
    let a = 42;
    make_copy(a);
}

fn take_ownership(s_arg: String) {
    println!("s_arg: {}", s_arg);
}

fn make_copy(a_arg: i32) {
    println!("a_arg: {}", a_arg);
}

/*
 | ----------------------------------------------------------------------------
 |   EXAMPLE 2
 | ----------------------------------------------------------------------------
 */
fn example2() {
    let s1 = give_ownership();
    let s2 = String::from("hello");
    // s2 ==(move)==> s_arg ==(move)==> s3
    let s3 = take_and_give_ownership(s2);
    // println!("s1: {}, s2: {}, s3: {}", s1, s2, s3); // s2 was dropped!
    println!("s1: {}, s3: {}", s1, s3);
    // s1: hell, s3: hello
}

fn give_ownership() -> String {
    let some_string = String::from("hell");
    some_string
}

fn take_and_give_ownership(s_arg: String) -> String {
    s_arg
}

/*
 | ----------------------------------------------------------------------------
 |   EXAMPLE 3
 | ----------------------------------------------------------------------------
 */
fn example3() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("s1: {}, len: {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main() {
    example1();
    // s_arg: hello
    // a_arg: 42

    example2();
    // s1: hell, s3: hello

    example3();
    // s1: hello, len: 5
}
