#![feature(iter_array_chunks)]
#![feature(array_windows)]

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
// pub mod day7;
// pub mod day8;
// pub mod day9;
// pub mod day10;
// pub mod day11;
// pub mod day12;
// pub mod day13;
// pub mod day14;
// pub mod day15;
// pub mod day16;
// pub mod day17;
// pub mod day18;
// pub mod day19;
// pub mod day20;
// pub mod day21;
// pub mod day22;
// pub mod day23;
// pub mod day24;
// pub mod day25;

#[macro_export]
macro_rules! test_day {
    ($day:literal, $sample_a_answer:literal, $sample_b_answer:literal) => {
        #[cfg(test)]
        pub mod tests {
            concat_idents::concat_idents!(name = day, $day {
                #[test]
                fn name() {
                    let sample = include_str!(concat!("../input/day", $day, ".sample"));
                    let input = include_str!(concat!("../input/day", $day, ".in"));

                    let result = super::part_a(sample);
                    assert_eq!(result.to_string(), $sample_a_answer.to_string());

                    let result = super::part_b(sample);
                    assert_eq!(result.to_string(), $sample_b_answer.to_string());

                    let result = super::part_a(input);
                    println!("{}", result.to_string());

                    let result = super::part_b(input);
                    println!("{}", result.to_string());
                }
            });
        }
    }
}
