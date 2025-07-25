fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sing_in_count: u64,
    }

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sing_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    fn build_user(email: String, username: String) -> User {
        User {
            activate: true,
            username: username,
            email: email,
            sign_in_count: 1,
        }
    }
}
