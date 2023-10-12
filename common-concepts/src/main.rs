fn main() {
    let mut a = String::from("value sami tests");
    let r1 = &a;
    let r2 = &a;
    let r3 = &a;

    // let ab = first_word(&a);

    // prinln!("{ab}");
    let ax = second_first_word(&a);
    println!("{ax}");
    println!("{a}");
}

fn fibonaci(number: i32) -> i32 {
    if number <= 1 {
        return 1;
    }

    return fibonaci(number - 1) + fibonaci(number - 2);
}
// 1 2 3 5 8 13 21

// fn celsius(grade: f32) -> f32 {
//     return grade * 4.8 + 32.0;
// }
//
// fn modified(ida: &String) -> usize {
//     ida.len()
// }
//
// fn modified2(ida: &mut String) {
//     ida.push_str("string")
// }
//
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

fn second_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
