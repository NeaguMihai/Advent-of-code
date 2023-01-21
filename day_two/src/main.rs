fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let file_path = &args[0];
    let file_content = read_file(file_path);
    let res = file_content.lines().fold(0, |acc, x: &str| {
        let (oponent, response) = x.split(" ").collect::<(&str, &str)>();
        let oponent = RockPaperScissors::from(oponent);
        let response = RockPaperScissors::from(response);
        acc + RockPaperScissors.matchPoints(response, oponent) + RockPaperScissors.getPoints(response);
        acc 
    });
    println!("{res}")
}

fn read_file(path: &String) -> String {
    let content = fs::read_to_string(path);
    match content {
        Ok(f) => {
            return f;
        }
        Err(err) => {
            panic!("Error: {}", err)
        }
    }
}

enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl RockPaperScissors {
    fn from(s: &str) -> RockPaperScissors {
        match s {
            "A" => RockPaperScissors::Rock,
            "B" => RockPaperScissors::Paper,
            "C" => RockPaperScissors::Scissors,
            "X" => RockPaperScissors::Rock,
            "Y" => RockPaperScissors::Paper,
            "Z" => RockPaperScissors::Scissors,
        }
    }

    fn getPoints(current) -> u32 {
        match current {
            RockPaperScissors::Rock => 1,
            RockPaperScissors::Paper => 2,
            RockPaperScissors::Scissors => 3,
        }
    }

    fn matchPoints(current, oponent: &RockPaperScissors) -> u32 {
        match current {
            RockPaperScissors::Rock => match oponent {
                RockPaperScissors::Scissors => 6,
                RockPaperScissors::Rock => 3,
                RockPaperScissors::Paper => 0,
            },
            RockPaperScissors::Paper => match oponent {
                RockPaperScissors::Rock => 6,
                RockPaperScissors::Paper => 3,
                RockPaperScissors::Scissors => 0,
            },
            RockPaperScissors::Scissors => match oponent {
                RockPaperScissors::Paper => 6,
                RockPaperScissors::Scissors => 3,
                RockPaperScissors::Rock => 0,
            },
        }
    }
}