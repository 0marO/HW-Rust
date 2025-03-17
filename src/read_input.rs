use std::io;


fn main() {


    println!("Please enter your name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");


    println!("Hello, {}", name);

    println!("Hello, {}", name.trim());
    println!("Hello, {}", name.trim().to_uppercase());
    println!("Hello, {}", name.trim().to_lowercase());
    println!("Hello, {}", name.trim().to_uppercase().to_lowercase());
    println!("Hello, {}", name.trim().to_lowercase().to_uppercase()); 

}
