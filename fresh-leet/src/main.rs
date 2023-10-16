use std::collections::HashSet;
use std::cmp;

fn main() {
    let mut roman_string= String::from("LVIIIV");
    println!("{:?}", convert_roman_in_int(&roman_string));
}

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut c_vec: Vec<i32> = [nums1, nums2].concat();
    c_vec.sort();

    let c_len = (c_vec.len() - 1) >> 1;
    println!("{:?}", (c_vec.len() - 1 >> 1));
    (c_vec[c_len] + c_vec[c_len + (c_vec.len() - 1 & 1)]) as f64 / 2.0
}


fn is_palindrome(x: i32) -> bool {
    let binding = x.to_string();
    let num_str = binding.as_bytes();
    let mut max = binding.len() - 1;
    for _index in 0..num_str.len() {
        if binding.chars().nth(_index).unwrap() == binding.chars().nth(max).unwrap() {
            if max > 0 {
                max -= 1;
            }
        } else {
            return false;
        }
    }
    true
}

fn is_pal(x: i32) -> bool {
    x.to_string().chars().rev().eq(x.to_string().chars())
}

fn convert_roman_in_int(s: &String) -> i32 {
    s
        .replace("IV", "IIII")
        .replace("IX", "VIIII")
        .replace("XL", "XXXX")
        .replace("XC", "LXXXX")
        .replace("CD", "CCCC")
        .replace("CM", "DCCCC").chars().map(|c| {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }).sum()
}