#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
fn main() {
    let _user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    let _user2 = User {
        active: false,
        .._user1
    };
}
