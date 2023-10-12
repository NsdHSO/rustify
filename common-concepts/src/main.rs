use std::str::FromStr;

fn main() {
    let mut andrei = generate_user(String::from("Andrei"),String::from("andrei@gmai.com"));

    println!("{:?}", &andrei  )
}

#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    email:String
}

fn generate_user(name: String, email: String) -> User{
    User { name, age:0, email }
}