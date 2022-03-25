struct User {
    // <-- This is a struct definition
    username: String, // username is a **field**, String is the field's type
    email: String,
    sign_in_count: u64,
    is_active: bool,
}

fn new_user(email: String, username: String) -> User {
    User {
        email, // Equivalent to email: email,
        username,
        /// Equivalent to username: username,
        is_active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // This is a mutable instance
    let mut woody = User {
        // <-- This is an instance of User struct
        username: String::from("woody"),
        email: String::from("woody@example.com"),
        sign_in_count: 1,
        is_active: true,
    };

    // Update username
    woody.username = String::from("woody_edit");

    // This is an immutable instance
    let buzz = User {
        username: String::from("buzz"),
        email: String::from("buzz@example.com"),
        ..woody // <-- This means "copy all not-explicitly-assigned values from user1"
    };

    // This is an immutable instance created via a function
    let bo_peep = new_user(String::from("bo_peep@example.com"), String::from("bo_peep"));

    println!("{} <{}>", woody.username, woody.email); // woody_edit <woody@example.com>
    println!("{} <{}>", buzz.username, buzz.email); // buzz <buzz@example.com>
    println!("{} <{}>", bo_peep.username, bo_peep.email); // bo_peep <bo_peep@example.com>
}
