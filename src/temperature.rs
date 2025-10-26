fn main() {
    let farenheit = degrees_to_farenheit(37);
    let degrees = farenheit_to_degrees(79);

    println!("{}", farenheit);
    println!("{}", degrees);

    let sum = fibonnacci(2);

    println!("fibonnacci sum: {}", sum);
}

fn degrees_to_farenheit(degrees: i32) -> i32 {
    (degrees * 9 / 5) + 32
}

fn farenheit_to_degrees(farenheit: i32) -> i32 {
    (farenheit - 32) * 5 / 9
}

fn fibonnacci(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        fibonnacci(n - 1) + fibonnacci(n - 2)
    }
}
