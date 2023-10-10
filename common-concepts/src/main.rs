fn main() {
    let mut total = 0;
    for fib in 1..7  {
        total += fibonaci(fib);
    }
    println!("{total}");

    let romania = celsius(30.0);
    println!("{}", romania)
}

fn fibonaci (number: i32)-> i32 {
    if(number <=1){
        return 1;
    }
    return fibonaci(number-1) + fibonaci(number-2);
}
// 1 2 3 5 8 13 21

fn celsius(grade:f32) -> f32{
    return grade * 1.8 + 32.0;
}