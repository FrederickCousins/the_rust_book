
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
        username, // using field init shorthand
        email,
        sign_in_count: 1,
    }
}

// Tuple structs are essentially structs without fieldnames:
struct Colour(i32, i32, i32);

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername1"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // user1.active = false;
    // Compilation fails as user1 is immutable
    // unless prefixed with mut in the original declaration:

    let mut user2 = User {
        active: true,
        username: String::from("someusername2"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user2.active = false;
    dbg!(user2);

    let user3 = build_user(String::from("hi@aol.com"), String::from("hello"));

    let user4 = User {
        email: String::from("user3hasanew@email.com"),
        ..user3
    };

    dbg!(user4);

    let colour1 = Colour(233, 45, 23);
}
