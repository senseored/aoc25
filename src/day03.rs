use std::fs;
pub fn main(file_path: &str) {
    // let file_path = "input/test03.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let parts = populate(&contents);

    println!("part 1: {}", parts.0);
    println!("part 2: {}", parts.1);
}

fn populate(contents: &str) -> (u128, u128) {
    let mut part1 = 0;
    let mut part2 = 0;
    contents.lines().for_each(|line| {
        let mut p1 = 0;
        let mut p1pos = 0;
        let mut p2 = 0;
        line.chars().enumerate().for_each(|(i, char)| {
            let c = char::to_digit(char, 10).unwrap();
            if i < line.len() - 1 {
                if p1 < c as u128 {
                    p1 = c as u128;
                    p1pos = i;
                }
            }
        });
        for i in p1pos + 1..line.len() {
            let c = line.chars().nth(i).unwrap().to_digit(10).unwrap();
            if p2 < c as u128 {
                p2 = c as u128;
            }
        }
        // println!("{} {}", p1, p2);
        part1 = (p1 * 10) + p2 + part1;

        let mut p2string = String::new();

        for i in 0..11 {
            line.chars()
                .enumerate()
                .for_each(|(j, char)| if j < line.len() - (12 - i) {});
        }
    });

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let file_path = "input/test03.txt";

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        let test = populate(&contents);

        assert_eq!(test, (357, 0));
    }
}
