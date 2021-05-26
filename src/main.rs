

fn main() {
    use std::env;
    
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
}
