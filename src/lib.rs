use std::error::Error;

pub struct Config {
    pub num_of_tower: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("필요한 인수를 입력하지 않았습니다.");
        }
        let num_of_tower = &args[1].clone();

        Ok(Config {
            num_of_tower: num_of_tower.to_string(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = config.num_of_tower;
    println!("\n탑 높이:\n{}", contents);
    Ok(())
}
