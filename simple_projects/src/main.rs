pub mod reverse_string;
fn main() {
    let name = String::from("Brian");
    println!("{}", reverse_string::string_reversal(&name))
}
