use std::fs;
pub fn main(file_path: &str) {
    // let file_path = "input/test04.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let parts = populate(&contents);

    println!("part 1: {}", parts.0);
    println!("part 2: {}", parts.1);
}

fn populate(contents: &str) -> (u32, u32) {
    let (mut part1, mut part2) = (0, 0);
    let mut chars: Vec<Vec<char>> = Vec::new();
    let mut width = 0;
    let height = contents.lines().count();
    contents.lines().for_each(|line| {
        chars.push(line.chars().collect());
        width = chars.len();
    });

    let mut p2chars = chars.clone();

    for i in 0..height {
        for j in 0..width {
            let c = chars[i][j];
            if c == '@' {
                let mut rolls = 0;
                let dirs = dirs((i as u32, j as u32), width as u32, height as u32);
                for dir in dirs {
                    if chars[dir.0 as usize][dir.1 as usize] == '@' {
                        rolls += 1;
                    }
                }
                if rolls < 4 {
                    part1 += 1;
                    p2chars[i][j] = 'X';
                    part2 += 1;
                }
            }
        }
    }
    let mut unchanged = false;
    while !unchanged {
        chars = p2chars.clone();
        let mut p2check = part2;
        for i in 0..height {
            for j in 0..width {
                let c = chars[i][j];
                if c == '@' {
                    let mut rolls = 0;
                    let dirs = dirs((i as u32, j as u32), width as u32, height as u32);
                    for dir in dirs {
                        if chars[dir.0 as usize][dir.1 as usize] == '@' {
                            rolls += 1;
                        }
                    }
                    if rolls < 4 {
                        p2chars[i][j] = 'X';
                        part2 += 1;
                    }
                }
            }
        }
        if p2check == part2 {
            unchanged = true;
        }
    }
    (part1, part2)
}

fn dirs(pos: (u32, u32), width: u32, height: u32) -> Vec<(u32, u32)> {
    let mut dirs = Vec::new();
    if pos.0 > 0 {
        dirs.push((pos.0 - 1, pos.1));
        if pos.1 > 0 {
            dirs.push((pos.0 - 1, pos.1 - 1));
        }
        if pos.1 < height - 1 {
            dirs.push((pos.0 - 1, pos.1 + 1));
        }
    }
    if pos.0 < width - 1 {
        dirs.push((pos.0 + 1, pos.1));
        if pos.1 > 0 {
            dirs.push((pos.0 + 1, pos.1 - 1));
        }
        if pos.1 < height - 1 {
            dirs.push((pos.0 + 1, pos.1 + 1));
        }
    }
    if pos.1 > 0 {
        dirs.push((pos.0, pos.1 - 1));
    }
    if pos.1 < height - 1 {
        dirs.push((pos.0, pos.1 + 1));
    }
    dirs
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let file_path = "input/test04.txt";

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        let test = populate(&contents);

        assert_eq!(test, (13, 43));
    }
}
