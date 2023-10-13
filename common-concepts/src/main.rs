use std::str::FromStr;

fn main() {
    let mut andrei = generate_user(String::from("Andrei"), String::from("andrei@gmai.com"));
    let mut vracu = generate_user(String::from("Andrei"), String::from("andrei@gmai.com"));
    vracu.age = 4;
    andrei.age = 5;
    
    println!("Total age from Andrei and vracu is {}", User::addTowUser(&andrei, &vracu));
    println!("{:?}", &andrei);

    let verdict_for_notification: &str = send_notificaiton(&andrei);
    println!("{}", verdict_for_notification);
    andrei.add();
}

#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    email: String,
}

impl User{
    fn add(&self){
        println!("Hello")
    }

    fn addTowUser(one: &User,other: &User) -> i32{
        one.age + other.age
    }
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
