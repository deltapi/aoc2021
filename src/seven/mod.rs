use itertools::Itertools;
use num_integer::Integer;

fn constant_fuel(input: &str) -> usize {
    let intermediate = input.split(',').into_iter().map(|c| c.parse::<usize>().unwrap());
    let size = intermediate.clone().count();
    let median = intermediate.clone().sorted().skip(size.div_floor(&2)).take(1).find(|_| true).expect("hä?");

    intermediate.map(|x| (x as isize - median as isize).abs() as usize).sum()
}

fn increasing_fuel_brute(input: &str) -> usize {
    let intermediate = input.split(',').into_iter().map(|c| c.parse::<usize>().unwrap());
    let min = intermediate.clone().min().unwrap();
    let max = intermediate.clone().max().unwrap();

    (min..=max).into_iter().map(|i| {
        (intermediate.clone().map(|x| (x as isize - i as isize).abs()*((x as isize - i as isize).abs() +1) as isize).sum::<isize>()/2) as usize
    }).min().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_simple() {
        assert_eq!(constant_fuel(include_str!("./test_input")), 37);
    }

    #[test]
    fn input_simple() {
        assert_eq!(constant_fuel(include_str!("./input")), 2);
    }

    #[test]
    fn sample_input() {
        assert_eq!(increasing_fuel_brute(include_str!("./test_input")), 5);
    }

    #[test]
    fn input() {
        assert_eq!(increasing_fuel_brute(include_str!("./input")), 5);
    }

}