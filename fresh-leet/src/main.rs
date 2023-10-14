use std::collections::HashSet;

fn main() {

    length_of_longest_substring(String::from("iuiffca"));
}

fn length_of_longest_substring(s: String) -> u8{
    let mut start: usize = 0;
    let mut max = 0;
    let mut map: HashSet<char> = HashSet::new();
    for c in s.chars() {
        while map.get(&c).is_some() {
            map.remove(&s.chars().nth(start).unwrap());
            start += 1;
        }

        map.insert(c);
        max = std::cmp::max(max, map.len());
    }
    max as u8
}

