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
        ..user1
    };
}
