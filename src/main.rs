#[derive(Debug)]
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
        sign_in_count: 1
    }
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);
#[derive(Debug)]
struct AlwaysEqual;


fn main() {
    let mut emory = User {
        active: true,
        username: String::from("Emory.Du"),
        email: String::from("orangeduxiaocheng@gmail.com"),
        sign_in_count: 1,
    };

    println!("emory: {:?}", emory);

    emory.email = String::from("emorydu@163.com");

    println!("emory: {:?}", emory);

    let smith = build_user(
        String::from("smith@example.com"),
    String::from("smith"));

    println!("smith: {:#?}", smith);

    let joker = User {
        active: smith.active,
        username: String::from("joker"),
        email: String::from("joker@example.com"),
        sign_in_count: smith.sign_in_count,
    };

    println!("joker: {:?}", joker);

    let other = User{
        username: String::from("other"),
        email: String::from("other@example.com"),
        ..smith
    };

    println!("other: {:?}", other);

    let black = Color(0, 0, 0);
    println!("black: {:?}", black);
    let origin = Point(0, 0, 0);
    println!("origin: {:?}", origin);

    let subject = AlwaysEqual;
    println!("subject: {:?}", subject)

}