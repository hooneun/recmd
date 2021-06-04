extern crate recmd;

use recmd::config::Config;
use recmd::recmd::ReCmd;
use std::env;
use std::os::unix::prelude::CommandExt;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

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
        }
    } else if config.is_edit() {
        // TODO
        println!("edit");
    } else if config.is_help() {
        // TODO
        println!("help");
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
