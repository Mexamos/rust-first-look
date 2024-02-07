struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("example@gmail.com"),
        username: String::from("Ann"),
        active: true,
        sign_in_count: 1
    };
    println!("User email = {}", user1.email);

    let user2 = User {
        email: String::from("user2@gmail.com"),
        username: String::from("user2"),
        ..user1
    };
}

fn build_user (email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
