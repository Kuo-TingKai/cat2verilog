fn main() {
    println!("Simple test");
    let args: Vec<String> = std::env::args().collect();
    println!("Args: {:?}", args);
} 