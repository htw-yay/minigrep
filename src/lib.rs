use std::{error::Error, fmt::Display, fs};
pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub case_insensitive: bool,
}

#[derive(Debug)]
enum MyError {
    NotEnoughArguments,
}
impl Error for MyError {}
impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MyError::NotEnoughArguments => write!(f, "Not enough arguments"),
        }
    }
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Self, Box<dyn Error>> {
        if args.len() < 4 {
            return Err(Box::new(MyError::NotEnoughArguments));
        }
        Ok(Config {
            query: &args[1],
            file_path: &args[2],
            case_insensitive: args[3].parse()?,
        })
    }
    pub fn read(&self) -> Result<String, Box<dyn Error>> {
        Ok(fs::read_to_string(self.file_path)?)
    }
    pub fn search(&self, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();
        if self.case_insensitive {
            for line in contents.lines() {
                if line.to_lowercase().contains(&self.query.to_lowercase()) {
                    results.push(line);
                }
            }
        } else {
            for line in contents.lines() {
                if line.contains(&self.query) {
                    results.push(line);
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    #[test]
    fn one_result() {
        let config = Config {
            query: "jo",
            file_path: "",
            case_insensitive: false,
        };
        let contents = "\
Rust
yaya joe
omg";
        assert_eq!(vec!["yaya joe"], config.search(contents));
    }

    #[test]
    fn case_insensitive() {
        let config = Config {
            query: "rUST",
            file_path: "",
            case_insensitive: true,
        };
        let contents = "\
Rust
yaya joe
omg
trust me";
        assert_eq!(vec!["Rust", "trust me"], config.search(contents));
    }
}
