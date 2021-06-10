use std::collections::HashMap;
use std::env::consts;
use std::io::Error;
use std::fs::{create_dir, OpenOptions};

#[derive(Debug)]
pub struct ReCmd {
    pub cmd: HashMap<String, String>,
}

fn store_path() -> &'static str {
    let os = consts::OS;

    if os == "macos" {
        return "~/.config/recmd/store.json";
    }

    "./store.json"
}

impl ReCmd {
    pub fn new() -> Result<ReCmd, Error> {
        match create_dir("${HOME}/.config/recmd") {
            Ok(_) => println!("{}", "OK"),
            Err(e) => panic!("create dir filed {}", e),
        }

        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(store_path())?;

        match serde_json::from_reader(f) {
            Ok(cmd) => Ok(ReCmd { cmd }),
            Err(e) if e.is_eof() => Ok(ReCmd {
                cmd: HashMap::new(),
            }),
            Err(e) => panic!("{}", e),
        }
    }

    pub fn insert(&mut self, key: String, cmd: String) {
        self.cmd.insert(key, cmd);
    }

    pub fn save(self) -> Result<(), Error> {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .open(store_path())?;

        serde_json::to_writer_pretty(f, &self.cmd)?;
        Ok(())
    }

    pub fn read(&self, key: &String) -> Option<String> {
        match self.cmd.get(key) {
            Some(cmd) => Some(cmd.to_string()),
            None => None,
        }
    }

    pub fn delete(&mut self, key: &String) -> bool {
        true
    }

    pub fn view(&self) {
        if self.cmd.len() > 0 {
            println!("#\trecmd\tCommand");
            let mut cnt = 1;
            for (k, v) in self.cmd.iter() {
                println!("{}\t{}\t{}", cnt, k, v);
                cnt += 1;
            }

            return;
        }

        println!("recmd command not found");
    }
}
