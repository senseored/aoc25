use std::fs;
pub fn main(file_path: &str) {
    // let file_path = "input/test01.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let answer = solve(&contents);

    println!("part 1: {}", answer.0);
    println!("part 2: {}", answer.1);
}

fn solve(contents: &str) -> (i32, i32) {
    let mut answer: i32 = 0;
    let mut answer_p2: i32 = 0;
    let mut starting_pos: i32 = 50;
    contents.lines().for_each(|line| {
        if line.starts_with("R") {
            let number = line.strip_prefix("R").unwrap();
            for _ in 0..number.parse::<i32>().unwrap() {
                starting_pos += 1;
                starting_pos %= 100;
                if starting_pos == 0 {
                    answer_p2 += 1;
                }
            }
        } else {
            let number = line.strip_prefix("L").unwrap();
            for _ in 0..number.parse::<i32>().unwrap() {
                starting_pos -= 1;
                starting_pos %= 100;
                if starting_pos == 0 {
                    answer_p2 += 1;
                }
            }
        };
        if starting_pos == 0 {
            answer += 1;
        }
    });
    (answer, answer_p2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let file_path = "input/test01.txt";

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        let answer = solve(&contents);

        assert_eq!(answer, (3, 6));
    }
}
