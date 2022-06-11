pub struct Config {
    pub query: String,
    pub filename: String,
    pub show_all: bool,
}

pub struct Line {
    pub number: usize,
    pub content: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let show_all =
            args.contains(&"-sa".to_string()) || args.contains(&"--show-all".to_string());

        Ok(Config {
            query,
            filename,
            show_all,
        })
    }
}

impl Line {
    pub fn generate_report(&self) -> String {
        format!("[#{}] {}", (self.number + 1), self.content)
    }
}
