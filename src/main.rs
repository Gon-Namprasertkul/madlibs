fn main() {
    println!("Roses are ____");
    println!("Violets are ____");
    println!("Sugar is ____");
    println!("And so are ____");
    println!("");
    let mut color1 = String::new();
    let mut color2 = String::new();
    let mut taste = String::new();
    let mut person = String::new();
    println!("Enter the first color: ");
    std::io::stdin().read_line(&mut color1).expect("Failed to read line");
    println!("Enter the second color: ");
    std::io::stdin().read_line(&mut color2).expect("Failed to read line");
    println!("Enter the taste: ");
    std::io::stdin().read_line(&mut taste).expect("Failed to read line");
    println!("Enter a person: ");
    std::io::stdin().read_line(&mut person).expect("Failed to read line");

    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    print!("Roses are {} ", color1);
    print!("Violets are {}", color2);
    print!("Sugar is {}", taste);
    print!("And so are {}", person);
}
