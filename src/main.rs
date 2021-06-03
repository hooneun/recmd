extern crate recmd;

use recmd::config::Config;
use recmd::recmd::ReCmd;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("args: {:?}", args);

    let config = Config::new(&args).expect("invalid command");

    println!("config: {:?}", config);

    let mut recmd = ReCmd::new().expect("File Init Failed");
    println!("{:?}", recmd);
    if config.is_add() {
        recmd.insert(config.key, config.cmd);
        match recmd.save() {
            Ok(_) => println!("recmd saved"),
            Err(e) => println!("An error occurred: {}", e),
        }
    } else if config.is_edit() {
    } else {
        panic!("invalid command");
    }
}
