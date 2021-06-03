use std::collections::HashMap;

fn main() {
    use std::env;
    
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
}

struct Command {
    cmd: HashMap<String, String>,
}

impl Command {
    fn new() {

    }

    fn save() {

    }

    fn read() {

    }
}
