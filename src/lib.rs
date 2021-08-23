use std::error::Error;
use std::fs;

pub struct Config {
    pub num_of_tower: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("필요한 인수를 입력하지 않았습니다.");
        }
        let num_of_tower = &args[1].clone();
        let filename = &args[2].clone();

        Ok(Config {
            num_of_tower: num_of_tower.to_string(),
            filename: filename.to_string(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("\n읽은 파일 내용:\n{}", contents);
    Ok(())
}
