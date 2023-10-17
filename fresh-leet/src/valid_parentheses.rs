pub fn is_valid(s: String) -> bool {
    if s.len() & 1 == 1 {
        return false;
    }
    let mut v = Vec::with_capacity(s.len());

    for c in s.chars() {
        match c {
            '(' | '{' | '[' => v.push(c),
            _ => match v.pop() {
                None => {}
                Some('(') if c == ')' => (),
                Some('{') if c == '}' => (),
                Some('[') if c == ']' => (),
                _ => return false,
            },
        }
    }
    v.is_empty()
}
