#[allow(dead_code)]  // for the warning that the fields are never read
#[derive(Debug)]
struct User<'a, 'b> {
    active: bool,
    username: &'a str, // forces me to add lifetime spec
    email: &'b str, //  forces me to add lifetime spec
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username:  "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };

    println!("User1 = {:?}", user1);
}
