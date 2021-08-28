use std::error::Error;
use std::fmt::{Display, Formatter};

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

// https://gist.github.com/lovasoa/f265350ecd5aa7511235e2d122695725

pub struct Hanoi {
    disks: [Vec<u32>; 3],
    size: u32,
}

impl Hanoi {
    pub fn new(num_disks: u32) -> Self {
        Hanoi {
            size: num_disks,
            disks: [(1..=num_disks).rev().collect(), vec![], vec![]],
        }
    }

    fn move_disk(&mut self, from: usize, to: usize) {
        let d = self.disks[from].pop().unwrap();
        self.disks[to].push(d);
        println!("{}", self);
    }

    fn solve_at(&mut self, n: u32, from: usize, middle: usize, to: usize) {
        if n > 0 {
            self.solve_at(n - 1, from, to, middle);
            self.move_disk(from, to);
            self.solve_at(n - 1, middle, from, to);
        }
    }

    pub fn solve(&mut self) {
        println!("{}", self);
        self.solve_at(self.size, 0, 1, 2);
    }
}

impl Display for Hanoi {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let s = self.size as usize;
        for i in (0..s).rev() {
            for j in 0..(3 * s) {
                if j % s == 0 {
                    write!(f, "║")?;
                }
                write!(
                    f,
                    "{}",
                    match self.disks[j / s].get(i) {
                        Some(&n) if (n as usize) > (j % s) => '─',
                        _ => ' ',
                    }
                )?;
            }
            write!(f, "║\n")?;
        }
        Ok(())
    }
}
