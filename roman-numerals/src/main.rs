extern crate roman_numerals;

use std::env;
use roman_numerals::Roman;


fn main() {
    let args: Vec<String> = env::args().collect();
    let num: u32 = args[1].parse().unwrap();
    let roman = Roman::from(num);
    println!("{} -> {}", num, roman);
}
