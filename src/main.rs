mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let day = &args[1];
        match day.as_str() {
            "01" => day01::main("input/day_01.txt"),
            "02" => day02::main("input/day_02.txt"),
            "03" => day03::main("input/day_03.txt"),
            "04" => day04::main("input/day_04.txt"),
            "05" => day05::main("input/day_05.txt"),
            "06" => day06::main("input/day_06.txt"),
            "07" => day07::main("input/day_07.txt"),
            "08" => day08::main("input/day_08.txt"),
            "09" => day09::main("input/day_09.txt"),
            "10" => day10::main("input/day_10.txt"),
            "11" => day11::main("input/day_11.txt"),
            "12" => day12::main("input/day_12.txt"),
            "13" => day13::main("input/day_13.txt"),
            "14" => day14::main("input/day_14.txt"),
            "15" => day15::main("input/day_15.txt"),
            "16" => day16::main("input/day_16.txt"),
            "17" => day17::main("input/day_17.txt"),
            "18" => day18::main("input/day_18.txt"),
            "19" => day19::main("input/day_19.txt"),
            "20" => day20::main("input/day_20.txt"),
            "21" => day21::main("input/day_21.txt"),
            "22" => day22::main("input/day_22.txt"),
            "23" => day23::main("input/day_23.txt"),
            "24" => day24::main("input/day_24.txt"),
            "25" => day25::main("input/day_25.txt"),
            _ => println!("Unknown day"),
        }
    } else {
        println!("Please provide a day as an argument");
    }
}
