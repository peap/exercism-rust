pub fn reply(prompt: &str) -> String {
    let prompt_was_shouted = prompt.chars().fold(true, |acc, c| acc && !c.is_lowercase());
    if prompt == "" {
        "Fine. Be that way!".to_string()
    } else if prompt.ends_with("?") {
        "Sure.".to_string()
    } else if prompt_was_shouted {
        "Whoa, chill out!".to_string()
    } else {
        "Whatever.".to_string()
    }
}
