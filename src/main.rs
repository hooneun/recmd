extern crate recmd;

use recmd::recmd::ReCmd;

fn main() {
    use std::env;
    
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let recmd = ReCmd::new();

    println!("{:?}", recmd);
}
