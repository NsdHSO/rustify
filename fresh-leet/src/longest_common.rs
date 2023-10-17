pub mod longest {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        println!("empty  ,{:?}", strs.is_empty());

        match strs.is_empty() {
            true => "".to_string(),
            _ => strs.iter().skip(1).fold(strs[0].clone(), |acc, x| {
                acc.chars()
                    .zip(x.chars())
                    .take_while(|(x, y)| x == y)
                    .map(|(x, y)| x)
                    .collect()
            }),
        }
    }
}
