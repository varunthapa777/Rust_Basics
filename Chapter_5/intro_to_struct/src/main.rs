// Define struct

struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

// define struct tuple without name
struct Color(i32, i32, i32);

// define unit-like struct

struct AlwaysEqual;

fn main() {
    // create instance of a User

    let mut user_1 = User {
        username: String::from("varunthapa777"),
        email: String::from("varuthapa@gmail.com"),
        active: true,
        sign_in_count: 0,
    };

    // change data
    user_1.active = false;

    // access value of User instance
    println!("{}", user_1.username);
    println!("{}", user_1.email);
    println!("{}", user_1.active);
    println!("{}", user_1.sign_in_count);

    // create user using user_builder func

    let user_2 = user_builder(
        String::from("Arunthapa"),
        String::from("arunthapa@gmail.com"),
    );

    println!("user_2 : {}", user_2.username);

    // create instance using other instances with update syntax

    let user_3 = User {
        username: String::from("Vijuu"),
        ..user_1
    };

    println!("user_3 = {}", user_3.email);

    let black = Color(0, 0, 0);
    println!("color: black({}, {}, {})", black.0, black.1, black.2);
    let _subject = AlwaysEqual;
}

fn user_builder(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 0,
    }
}
