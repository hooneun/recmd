use std::collections::HashMap;

pub struct ReCmd {
    pub cmd: HashMap<String, String>,
}

impl ReCmd {
    pub fn new() -> Result<ReCmd, std::io::Error> {
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

    pub fn save() {}

    pub fn read() {}
}
