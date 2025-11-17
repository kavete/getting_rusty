pub fn degrees_to_farenheit(degrees: i32) -> i32 {
    (degrees * 9 / 5) + 32
}

pub fn farenheit_to_degrees(farenheit: i32) -> i32 {
    (farenheit - 32) * 5 / 9
}

pub fn fibonnacci(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        fibonnacci(n - 1) + fibonnacci(n - 2)
    }
}
