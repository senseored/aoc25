use std::fs;
pub fn main(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // solve(&contents);

    println!("{contents}");
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test() {
//         let file_path = "input/test01.txt";
//
//         let contents =
//             fs::read_to_string(file_path).expect("Should have been able to read the file");
//         let test = populate(&contents);
//
//         assert_eq!(test, (142, 0));
//     }
// }
