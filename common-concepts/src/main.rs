use std::io;
fn main() {
    let mut second = String::new();

    io::stdin().read_line(&mut second).expect("Failed to read on the keyboard");

    let second:u32 = match second.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error");
            return;
        }
    };

    let y = {
        let x = 4;
        x+1
    };
    println!("{}",y);
}