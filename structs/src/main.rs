struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone123"),
        active: true,
        sign_in_count: 1,
    };
    //we can do the below line, because the struct instance is immutable
    user1.email = String::from("someone_changed@example.com");
    //below shorthand notation copies the rest of the values from user1
    let user2 = User {
        email: String::from("someone_new@example.com"),
        ..user1
    };
}
