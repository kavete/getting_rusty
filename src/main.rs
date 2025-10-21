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

    println!("{}",my_double);
    println!("{}", my_string);
    println!("{}", my_bool);
    println!("{}", my_letter);

    println!("{} is {} years old", name, age);

    // Constants
    const BIRTH_YEAR: i32 = 2025;

    println!("Born {}", BIRTH_YEAR);

    // If statements

    let score = 87;

    if score >=90 {
        println!("Grade: A");
    } else if score >=80 {
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

    let  mut value = 1;

    let result1 = loop {
        println!("Yoh");

        if value == 3 {
            break value;
        }
        value +=1;
    };

    println!("Stop value: {}", result1);

    let mut num = 1;

    while num <=10 {
        println!("{num}");
        num +=1;
    }

    let mut num2 = 1;

    while num2 <= 10 {

        println!("{}", num2);

        if num2 == 6 {
            break;
    }
        num2 +=1;

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
        a +b
    }

    let sum = add(6, 7);

    println!("{sum}");


}
