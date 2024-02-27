struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        username: String::from("Shobhit Gupta"),
        email: String::from("shogupt2002@gmai.com"),
        sign_in_count: 1,
        active: true,
    };
    user1.sign_in_count = 2;
    println!("The value of user1 is: {}", user1.username);
    println!("The value of user1 is: {}", user1.email);
    println!("The value of user1 is: {}", user1.sign_in_count);
    println!("The value of user1 is: {}", user1.active);
}
