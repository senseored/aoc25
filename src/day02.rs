use std::fs;
pub fn main(file_path: &str) {
    let file_path = "input/test02.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let answer = populate(&contents);

    println!("part 1: {}", answer.0);
    println!("part 2: {}", answer.1);
}

fn populate(contents: &str) -> (u128, u128) {
    let (mut part1, mut part2) = (0, 0);
    let contents = contents.trim();
    contents.split(',').into_iter().for_each(|x| {
        let mut z: usize = 0;
        let mut n: usize = 0;
        x.split('-')
            .collect::<Vec<&str>>()
            .iter()
            .enumerate()
            .for_each(|(i, y)| {
                if i == 0 {
                    z = y.parse::<usize>().unwrap();
                } else {
                    n = y.parse::<usize>().unwrap();
                    for i in z..=n {
                        let value = i.to_string();
                        if value[0..value.len() / 2] == value[value.len() / 2..value.len()] {
                            part1 += i as u128;
                        }
                        let mut p2_temp: u128 = 0;
                        for m in 0..value.len() / 2 {
                            println!("{}", value);
                            if value[..m] == value[m + 1..(m + 1) * 2] {
                                p2_temp = value[0..m].parse::<u128>().unwrap();
                            }
                        }
                        part2 += p2_temp;
                    }
                }
            })
    });
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let file_path = "input/test02.txt";

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        let test = populate(&contents);

        assert_eq!(test, (1227775554, 4174379265));
    }
}
