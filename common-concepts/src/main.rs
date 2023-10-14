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

    println!("You have this coin {}", find_coin(Coin::Quarter));
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


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn find_coin(coin: Coin) -> u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 3,
        Coin::Dime => 5,
        Coin::Quarter => 7,
    }
}