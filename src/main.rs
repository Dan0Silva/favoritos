struct User {
    name: String,
    email: String
}

fn build_user(name: String, email: String) -> User {
    User {
        name,
        email
    }
}

fn main() {
    let user: User = build_user(String::from("Danilo A. Silva"), String::from("dan@mail"));

    print!("{} : {}\n", user.name, user.email);
}
