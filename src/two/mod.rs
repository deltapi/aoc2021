use std::str::FromStr;
use crate::little_helpers;
use crate::little_helpers::read_lines;

fn multiply_positions(input: &str) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;


        input
        .lines()
        .into_iter()
        .map(|x| x.to_string().split(' ').into_iter().map(|x| String::from(x)).collect::<Vec<String>>())
        .map(|x| (x[0].clone(), x[1].parse::<u32>().expect("parse Error")))
        .for_each(|(x, y)| match x.as_str() {
            "forward" => horizontal +=y,
            "up" => depth -=y,
            "down" => depth +=y,
            _ => (),
        });

    horizontal*depth
}

fn multiply_positions_with_aim(input: &str) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;


    input
        .lines()
        .into_iter()
        .map(|x| x.to_string().split(' ').into_iter().map(|x| String::from(x)).collect::<Vec<String>>())
        .map(|x| (x[0].clone(), x[1].parse::<u32>().expect("parse Error")))
        .for_each(|(x, y)| match x.as_str() {
            "forward" => {
                depth += aim * y;
                horizontal += y;
            },
            "up" => aim -=y,
            "down" => aim +=y,
            _ => (),
        });

    horizontal*depth
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_simple() {
        assert_eq!(multiply_positions(include_str!("./test_input")), 150);
    }

    #[test]
    fn real_input_simple() {
        assert_eq!(multiply_positions(include_str!("./input")), 1507611);
    }

    #[test]
    fn sample_input_simple_with_aim() {
        assert_eq!(multiply_positions_with_aim(include_str!("./test_input")), 900);
    }

    #[test]
    fn real_input_simple_with_aim() {
        assert_eq!(multiply_positions_with_aim(include_str!("./input")), 1880593125);
    }
}