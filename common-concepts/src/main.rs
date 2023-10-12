use std::str::FromStr;

fn main() {
    let mut andrei = generate_user(String::from("Andrei"), String::from("andrei@gmai.com"));

    println!("{:?}", &andrei);

    let verdict_for_notification: &str = send_notificaiton(&andrei);
    println!("{}", verdict_for_notification)
}

#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    email: String,
}

fn generate_user(name: String, email: String) -> User {
    User {
        name,
        age: 0,
        email,
    }
}

fn send_notificaiton(user: &User) -> &str {
    match user.age > 10 {
        true => {
            println!("Notification was sended!");
            "You are lucky"
        }
        false => {
            println!("No notification was sended!");
            "You badly"
        }
    }
}
