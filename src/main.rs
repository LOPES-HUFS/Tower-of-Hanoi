use std::env;
use std::process;

use tower::Config;
use tower::Hanoi;

fn main() {
    //println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("입력한 명령어에서 다음과 같은 문제가 발생했습니다: {}", err);
        process::exit(1);
    });
    println!("탑 높이: {}", config.num_of_tower);
    let num = config.num_of_tower.parse::<u32>().unwrap();
    Hanoi::new(num).solve();

    if let Err(e) = tower::run(config) {
        println!("어플리케이션 에러: {}", e);

        process::exit(1);
    }
}
