fn main() {
    println!("Testing Rust compilation...");
    let args: Vec<String> = std::env::args().collect();
    println!("Args: {:?}", args);
} 