use std::collections::HashMap;
use std::io::Error;

#[derive(Debug)]
pub struct ReCmd {
    pub cmd: HashMap<String, String>,
}

impl ReCmd {
    pub fn new() -> Result<ReCmd, Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("re.json")?;

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
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("re.json")?;

        serde_json::to_writer_pretty(f, &self.cmd)?;
        Ok(())
    }

    pub fn read(self, key: &String) -> Option<String> {
        match self.cmd.get(key) {
            Some(cmd) => Some(cmd.to_string()),
            None => None,
        }
    }
}
