use itertools::Itertools;
use nalgebra::DimAdd;
use itertools::izip;

fn only_vertical_and_horizontal(input: &str) -> usize {
    input.lines()
        .into_iter()
        .map(|line| to_vec_of_points(line, false))
        .flat_map(|v| v.into_iter())
        .into_group_map_by(|&x| x)
        .into_iter()
        .filter(|(key, values)| values.len() > 1)
        .count()
}

fn also_diagonal(input: &str) -> usize {
    input.lines()
        .into_iter()
        .map(|line| to_vec_of_points(line, true))
        .flat_map(|v| v.into_iter())
        .into_group_map_by(|&x| x)
        .into_iter()
        .filter(|(key, values)| values.len() > 1)
        .count()
}

fn to_vec_of_points(line: &str, consider_diags: bool) -> Vec<(usize, usize)> {
    let ((start_x, start_y), (end_x, end_y)) = line.split(' ')
        .filter(|&x| x != "->")
        .take(2)
        .map(|x| to_point(x))
        .collect_tuple()
        .expect("Failed to collect");

    if start_x == end_x {
        (usize::min(start_y, end_y)..=usize::max(start_y, end_y))
            .map(|y| (start_x, y))
            .collect::<Vec<(usize, usize)>>()
    } else if start_y == end_y {
        (usize::min(start_x, end_x)..=usize::max(start_x, end_x))
            .map(|x| (x, start_y))
            .collect::<Vec<(usize, usize)>>()
    } else if consider_diags {
         match (start_x < end_x, start_y < end_y)   {
             (true, true) => {
                 izip!((start_x..=end_x), (start_y..=end_y)).collect::<Vec<(usize, usize)>>()
             }
             (false, true) => {
                 izip!((end_x..=start_x), (start_y..=end_y).rev()).collect::<Vec<(usize, usize)>>()
             }
             (true, false) => {
                 izip!((start_x..=end_x), (end_y..=start_y).rev()).collect::<Vec<(usize, usize)>>()
             }
             (false, false) => {
                 izip!((end_x..=start_x), (end_y..=start_y)).collect::<Vec<(usize, usize)>>()
             }
         }
    } else { vec![] }
}

fn to_point(comma_separated_usize: &str) -> (usize, usize) {
    comma_separated_usize.split(',')
        .map(|c| c.parse::<usize>().expect("Parse error"))
        .collect_tuple()
        .expect("Failed to collect point")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_simple() {
        assert_eq!(only_vertical_and_horizontal(include_str!("./test_input")), 12);
    }

    #[test]
    fn input_simple() {
        assert_eq!(only_vertical_and_horizontal(include_str!("./input")), 5147);
    }

    #[test]
    fn sample_input() {
        assert_eq!(also_diagonal(include_str!("./test_input")), 12);
    }

    #[test]
    fn input() {
        assert_eq!(also_diagonal(include_str!("./input")), 16925);
    }

}