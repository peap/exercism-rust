pub fn raindrops(num: u32) -> String {
    let mut string = String::new();
    if num % 3 == 0 {
        string.push_str("Pling");
    }
    if num % 5 == 0 {
        string.push_str("Plang");
    }
    if num % 7 == 0 {
        string.push_str("Plong");
    }
    if string.len() == 0 {
        format!("{}", num)
    } else {
        string
    }
}
