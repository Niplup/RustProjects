struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: usize,
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!("username: {}", user.username);
    println!("email: {}", user.email);
    println!("active: {}", user.active);
    println!("sign in count: {}", user.sign_in_count);
    println!("\n");
}

fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        email: String::from("nipunshukla@email.com"),
        username: String::from("nipunshukla"),
        active: true,
        sign_in_count: 1,
    };

    print_user(&user1);

    user1.username = String::from("nipuns");

    print_user(&user1);

    let mut user2 = build_user(String::from("someonerandom@email.com"), String::from("someonerandom"));
    print_user(&user2);
    user2.email = String::from("someonenotrandom@email.com");
    user2.username = String::from("someonenotrandom");
    print_user(&user2);

    let user3 = User {
        email: String::from("example@email.com"),
        ..user1
    };
    
    print_user(&user3);

}
