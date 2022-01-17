struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("warrior01@email.com"),
        String::from("chaoswarrior"),
    );
    let user2 = User {
        email: String::from("hallo@mail.com"),
        username: String::from("warrior01"),
        ..user1 // update syntax normally it would be
        /*
        active: user1.active,
        username: user1.username,
        email: String::from("hallo@mail.com"),
        sign_in_count: user.sign_in_count,
        */
    };

    // Example of Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let mut origin = Point(0, 0, 0);
    origin.0 = 1;
    println!("{}",origin.0);

    // Unit-Like Structs Without Any Fields
    struct AlwaysEqual;

    let subject = AlwaysEqual;
}
