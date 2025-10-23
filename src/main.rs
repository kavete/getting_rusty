fn main() {
    println!("Hello, world!");
    print!("Hello Again!\n");

    let name = "Liz";
    let age = 2;
    // age = 3; - Not allowed
    let mut x = 5;
    println!("Before: x = {}", x);

    x = 10;

    println!("After: x = {}", x);

    let number_of_days: i32 = 44;
    println!("Days: {}", number_of_days);

    let my_double: f64 = 6.4;
    let my_string: &str = "Raila";
    let my_letter: char = 'A';
    let my_bool: bool = true;

    println!("{}", my_double);
    println!("{}", my_string);
    println!("{}", my_bool);
    println!("{}", my_letter);

    println!("{} is {} years old", name, age);

    // Constants
    const BIRTH_YEAR: i32 = 2025;

    println!("Born {}", BIRTH_YEAR);

    // If statements

    let score = 87;

    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else {
        println!("Grade: C");
    }

    // Operators
    let sum = 5 + 5;
    let mult = 76 * 34;

    println!("Sum: {sum}");
    println!("Multiple: {mult}");

    let time = 20;

    let greeting = if time < 18 {
        "Good day"
    } else {
        "Good evening"
    };

    println!("{}", greeting);
    // Match

    let day = 7;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thurday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day"),
    }

    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }
    // loops

    let mut count = 1;

    loop {
        println!("Rust: {}", count);
        if count == 3 {
            break;
        }
        count += 1;
    }

    let mut value = 1;

    let result1 = loop {
        println!("Yoh");

        if value == 3 {
            break value;
        }
        value += 1;
    };

    println!("Stop value: {}", result1);

    let mut num = 1;

    while num <= 10 {
        println!("{num}");
        num += 1;
    }

    let mut num2 = 1;

    while num2 <= 10 {
        println!("{}", num2);

        if num2 == 6 {
            break;
        }
        num2 += 1;
    }

    // For loops

    for i in 1..6 {
        println!("i is {}", i);
    }

    for i in 1..=6 {
        println!("i is {}", i);
    }

    // Functions

    fn say_hello() {
        println!("Rusty");
    }

    say_hello();

    fn greet(name: &str) {
        println!("Hello {}", name)
    }

    greet("Brian");

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let sum = add(6, 7);

    println!("{sum}");

    let greeting: &str = "Hello";

    println!("{}", greeting);

    let _ = "Hello Nigga!".to_string();
    let _ = String::from("Howdy Bi!");
    let mut howdy = String::from("Howdy ");

    howdy.push_str("Biyatch");

    howdy.push('!');

    println!("{}", howdy);

    //Concatenate
    let s1 = String::from("Yoh");
    let s2 = String::from("Nigga");
    let s3 = String::from("What's up");

    let result = format!("{} {} {}", s1, s2, s3);

    let plus_result = s1 + " " + &s2 + " " + &s3;

    // let err_result = s1 + " " + s2 + " " + s3;

    println!("{}", result);
    println!("{}", plus_result);

    println!("Length: {}", result.len());

    // Ownership

    let a = String::from("Owner");
    let b = a; // b now owns the string

    println!("{}", b);

    // Simple types are coppied

    let number = 5;
    let number_copy = number;

    println!("{}", number);
    println!("{}", number_copy);

    //Cloning

    let some_string = String::from("Some String");

    let clone_some_string = some_string.clone();


    println!("{}", some_string);
    println!("{}", clone_some_string);


    // Borrowing
    // Using a value without taking ownership of it

    let some_text = String::from("Some text");
    let borrowed_text = &some_text;


    println!("{}", some_text);
    println!("{}", borrowed_text);
    
    //Mutable reference

    let mut some_other_text = String::from("Some other text");
    let borrowed_some_other_text = &mut some_other_text;

    borrowed_some_other_text.push_str("Added Text");

    println!("{}", borrowed_some_other_text);


}
