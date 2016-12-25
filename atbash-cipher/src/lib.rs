pub fn encode(message: &str) -> String {
    let mut i: usize = 1;
    let mut encoded = String::new();
    for c in message.chars() {
        let chr = c.to_lowercase().next().unwrap();
        if chr >= 'a' && chr <= 'z' {
            let code = 'z' as u8 - chr as u8 + 'a' as u8;
            encoded.push(code as char);
        } else if chr.is_numeric() {
            encoded.push(chr);
        } else {
            continue;
        }
        if i % 5 == 0 {
            encoded.push(' ');
        }
        i += 1;
    }
    if encoded.ends_with(' ') {
        encoded.pop();
    }
    encoded
}

pub fn decode(message: &str) -> String {
    encode(message).replace(" ", "")
}
