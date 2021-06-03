use std::collections::HashMap;
pub struct ReCmd {
    cmd: HashMap<String, String>,
}

impl ReCmd {
    fn new() -> Result<Command, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("re.json")?;

        match serde_json::from_reader(f) {
            Ok(cmd) => Ok(Command { cmd }),
            Err(e) if e.is_eof() => Ok(Command {
                cmd: HashMap::new(),
            }),
            Err(e) => panic!("{}", e),
        }
    }

    fn save() {}

    fn read() {}
}
