pub fn string_reversal(some_string: &String) -> String {
    let mut reversed_string = String::new();

    for c in some_string.chars().rev() {
        reversed_string.push(c)
    }

    reversed_string
}
