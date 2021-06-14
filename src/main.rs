extern crate recmd;

use recmd::config::Config;
use recmd::doc::{ABOUT, HELP};
use recmd::recmd::ReCmd;
use std::env;
use std::os::unix::prelude::CommandExt;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("OS: {}", env::consts::OS);
    println!("args: {:?}", args);

    let config = Config::new(&args).expect("invalid command");

    println!("config: {:?}", config);

    let mut recmd = ReCmd::new().expect("File Init Failed");

    if config.is_add() {
        println!("add");
        recmd.insert(config.key, config.cmd);
        match recmd.save() {
            Ok(_) => println!("recmd saved"),
            Err(e) => println!("An error occurred: {}", e),
        };
    } else if config.is_delete() {
        println!("delete");
        recmd.delete(&config.key);
        match recmd.save() {
            Ok(_) => println!("recmd Deleted"),
            Err(e) => println!("An error occurred: {}", e),
        }
    } else if config.is_help() {
        println!("{} \n {}", ABOUT, HELP);
    } else if config.is_list() {
        recmd.view();
    } else if config.is_exec() {
        let cmd: String = match recmd.read(&config.cmd) {
            None => panic!("'{}' is not present in the list", config.cmd),
            Some(cmd) => cmd,
        };

        // splite string insert arg
        let cmd_args: Vec<&str> = cmd.split(" ").collect();
        println!("{:?}", cmd_args);
        let mut command = Command::new(cmd_args[0]);

        if cmd_args.len() > 1 {
            command.args(&cmd_args[1..]);
        }
        command.exec();
    }
}
