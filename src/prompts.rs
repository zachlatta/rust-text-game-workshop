use std::io;

fn prompt() -> bool {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    return input.to_ascii_lowercase().starts_with("y");
}

pub fn intro() {
    println!("Hello brave explorer! Want to embark on an adventure?");

    let answer = prompt();

    if answer == true {
        println!("Yay! Your aspirations will give you some very good karma later in your life.");
    } else {
        println!("Boo, you're such a buzzkill!");
    }
}
