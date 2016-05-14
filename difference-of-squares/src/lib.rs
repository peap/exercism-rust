pub fn square_of_sum(num: i32) -> i32 {
    (1..(num+1)).fold(0, |acc, x| acc + x).pow(2)
}

pub fn sum_of_squares(num: i32) -> i32 {
    (1..(num+1)).fold(0, |acc, x| acc + x.pow(2))
}

pub fn difference(num: i32) -> i32 {
    square_of_sum(num) - sum_of_squares(num)
}
