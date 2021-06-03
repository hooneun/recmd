#[derive(Debug)]
pub struct Config {
    kind: String,
    key: String,
    cmd: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, std::io::Error> {
        // TODO validate
        let kind = args[1].clone();
        let key = args[2].clone();
        let cmd = args[3].clone();

        Ok(Config { kind, key, cmd })
    }

    pub fn is_add(&self) -> bool {
        self.kind == "add"
    }

    pub fn is_edit(&self) -> bool {
        self.kind == "edit"
    }

}
