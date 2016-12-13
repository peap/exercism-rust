pub fn verse(num: i32) -> String {
    if num > 1 {
        let plural = if num == 2 { "bottle" } else { "bottles" };
        format!("{} bottles of beer on the wall, {} bottles of beer.\n\
                Take one down and pass it around, {} {} of beer on the wall.\n",
                num, num, num - 1, plural)
    } else if num == 1 {
        format!("1 bottle of beer on the wall, 1 bottle of beer.\n\
                Take it down and pass it around, no more bottles of beer on the wall.\n")
    } else {
        format!("No more bottles of beer on the wall, no more bottles of beer.\n\
                Go to the store and buy some more, 99 bottles of beer on the wall.\n")
    }
}

pub fn sing(from: i32, to: i32) -> String {
    let mut lyrics = String::new();
    let mut n = from;
    while n >= to {
        if n != from {
            lyrics.push('\n');
        }
        lyrics.push_str(&verse(n));
        n -= 1;
    }
    lyrics
}
