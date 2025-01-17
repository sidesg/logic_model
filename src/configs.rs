
pub struct Config {
    infile: String
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let infile = match args.next() {
            Some(pth) => pth,
            None => return Err("Should have argument with input file path")
        };

        Ok(Config{infile})
    }

    pub fn infile(&self) -> &str {
        &self.infile
    }
}
