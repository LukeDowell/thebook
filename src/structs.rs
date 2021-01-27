pub fn run() {
    // In rust, either the entirety of a struct is mutable or it isn't
    struct User {
        name: String,
        active: bool
    }

    let mut user1 = User {
        name: String::from("Luke"),
        active: false,
    };
    user1.active = true;

    // Same syntax sugar that javascript has with fields sharing parameter names
    fn build_user(name: String) -> User {
        User {
            name,
            active: false
        }
    }

    // Struct update syntax
    let user2 = User {
        name: String::from("Garbage"),
        ..user1
    };

    // Tuple structs
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    // Unit-like struct
    struct Validated;

    // The use of String inside a struct instead of &str is deliberate.
    // We want instances of a given struct to own all of it's data.
    // We want all of it's data to be valid as long as the struct itself is valid.

    example_structs_program();
}

fn example_structs_program() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}