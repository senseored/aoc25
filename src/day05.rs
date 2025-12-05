use std::collections::HashSet;
use std::fs;

pub fn main(file_path: &str) {
    // let file_path = "input/test05.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let parts = populate(&contents);

    println!("Part 1: {}", parts.0);
    println!("Part 2: {}", parts.1);
}

fn populate(contents: &str) -> (u128, u128) {
    let (fresh, ingredients) = contents.split_once("\r\n\r\n").unwrap();

    let ingredients = ingredients
        .lines()
        .map(|line| line.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();
    let mut p1bool: Vec<bool> = vec![false; ingredients.len()];
    let mut p2hash: HashSet<(u128, u128)> = HashSet::new();
    let mut p2: Vec<(u128, u128)> = Vec::new();
    if p2.is_empty() {
        p2.push((0, 0));
    }

    fresh.lines().enumerate().for_each(|(z, line)| {
        // println!("{}", z);
        let (a, b) = line.split_once("-").unwrap();
        let a = a.parse::<u128>().unwrap();
        let b = b.parse::<u128>().unwrap();
        let mut newp2 = false;
        p2.clone().iter().enumerate().for_each(|(j, (min, max))| {
            if (a < *min && b < *min) || (a > *max && b > *max) {
                // p2hash.insert((a, b));
                newp2 = true;
                // p2.push((a, b));
            } else if a <= *min && b >= *max {
                // p2hash.insert((a, b));
                p2[j] = (a, b);
                newp2 = false;
            } else if a >= *min && a <= *max && b >= *max {
                // p2hash.insert((*min, b));
                p2[j] = (*min, b);
                newp2 = false;
            } else if a <= *min && b >= *min && b <= *max {
                // p2hash.insert((a, *max));
                p2[j] = (a, *max);
                newp2 = false;
            }
        });
        if newp2 {
            // println!("new p2, {} - {}", a, b);
            p2.push((a, b));
        }
        // for x in a..=b {
        //     p2vec.insert(x);
        // }
        ingredients.iter().enumerate().for_each(|(n, i)| {
            if a <= *i && *i <= b {
                p1bool[n] = true;
            }
        });
        // println!("{}", z);
    });
    // p2.sort();
    p2.remove(0);
    let mut newlen = p2.len() - 1;
    while newlen != p2.len() {
        p2.sort();
        newlen = p2.len();
        p2 = prune_p2(p2);
        // println!("{}", p2.len());
    }
    println!("{}", p2.len());
    p2 = prune_p2(p2);
    println!("{}", p2.len());
    p2 = prune_p2(p2);
    println!("{}", p2.len());
    p2 = prune_p2(p2);
    println!("{}", p2.len());
    p2 = prune_p2(p2);
    println!("{}", p2.len());
    p2 = prune_p2(p2);
    println!("{}", p2.len());
    p2 = prune_p2(p2);
    println!("{}", p2.len());
    p2 = prune_p2(p2);
    println!("{}", p2.len());
    p2 = prune_p2(p2);
    println!("{}", p2.len());
    p2 = prune_p2(p2);
    println!("{}", p2.len());
    p2 = prune_p2(p2);
    // println!("{}", p2.len());
    // p2 = prune_p2(p2);
    // p2 = prune_p2(p2);
    println!("{:?}", p2);
    println!("{}", p2.len());

    let part1 = p1bool.iter().filter(|b| **b).count() as u128;
    let mut part2 = 0;
    p2.iter().for_each(|(a, b)| {
        part2 += 1;
        part2 += b - a;
    });

    (part1, part2)
}

fn prune_p2(p2in: Vec<(u128, u128)>) -> Vec<(u128, u128)> {
    let mut p2 = Vec::new();
    if p2.is_empty() {
        p2.push((0, 0));
    }
    p2in.iter().for_each(|(a, b)| {
        let mut newp2 = false;
        p2.clone().iter().enumerate().for_each(|(j, (min, max))| {
            if (*a < *min && *b < *min) || (*a > *max && *b > *max) {
                newp2 = true;
            }
            if *a <= *min && *b >= *max {
                p2[j] = (*a, *b);
                newp2 = false;
            } else if *a >= *min && *a <= *max && *b >= *max {
                p2[j] = (*min, *b);
                newp2 = false;
            } else if *a <= *min && *b >= *min && *b <= *max {
                p2[j] = (*a, *max);
                newp2 = false;
            }
        });
        if newp2 {
            p2.push((*a, *b));
        }
    });
    // p2.sort();
    p2.remove(0);
    let mut remlist: Vec<usize> = Vec::new();
    p2.clone().iter().enumerate().for_each(|(x, p)| {
        p2.clone().iter().enumerate().for_each(|(y, i)| {
            // let mut deletex = false;
            // let mut deletey = false;
            // println!("{} - {}, x:{}, y:{}", p.1, i.0, x, y);
            if p.1 == (i.0 - 1) && y != x {
                if p.1 < i.1 {
                    p2[x] = (p.0, i.1);
                    remlist.push(y);
                }
            }
            if p.0 == (i.1 + 1) && y != x {
                if p.0 < i.0 {
                    p2[x] = (i.0, p.1);
                    remlist.push(y);
                }
            }
        });
    });
    remlist.sort();
    remlist.iter().rev().for_each(|x| {
        p2.remove(*x);
    });
    p2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let file_path = "input/test05.txt";

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        let test = populate(&contents);

        assert_eq!(test, (3, 14));
    }
}
