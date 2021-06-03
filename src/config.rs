#[derive(Debug)]
pub struct Config {
    kind: String,
    pub key: String,
    pub cmd: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, std::io::Error> {
        let mut kind = "";
        let mut key:String = String::new();
        let mut cmd:String = String::new();

        if args.len() == 4 {
            if args[1] == "a" {
                kind = "add";
            } else if args[1] == "e" {
                kind = "edit";
            }
            key = args[2].clone();
            cmd = args[3].clone();
        } else if args.len() == 2 {
            if args[1] == "h" || args[1] == "help" {
                kind = "help"
            } else {
                kind = "exec";
                cmd = args[1].clone();
            }
        } else {
            panic!("invalid command");
        }

        Ok(Config {
            kind: kind.to_string(),
            key: key.to_string(),
            cmd: cmd.to_string(),
        })
    }

    pub fn is_add(&self) -> bool {
        self.kind == "add"
    }

    pub fn is_edit(&self) -> bool {
        self.kind == "edit"
    }

    pub fn is_help(&self) -> bool {
        self.kind == "help"
    }

    pub fn is_exec(&self) -> bool {
        self.kind == "exec"
    }
}
