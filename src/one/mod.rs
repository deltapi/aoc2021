use std::str::FromStr;
use crate::little_helpers;
use crate::little_helpers::read_lines;

fn simple_slide(input: String) -> usize {
    if let Ok(lines) = read_lines(input) {
        lines.filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .map(|x| u32::from_str(x.as_str()).expect("Parse Error"))
            .collect::<Vec<u32>>()
            .as_slice()
            .windows(2)
            .map(|x| x[0] < x[1])
            .filter(|&b| b)
            .count()
    } else { panic!("boah") }
}

fn window_sum(input: String) -> usize {
    if let Ok(lines) = read_lines(input) {
        lines.filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .map(|x| u32::from_str(x.as_str()).expect("Parse Error"))
            .collect::<Vec<u32>>()
            .as_slice()
            .windows(3)
            .map(|x| x.iter().sum())
            .collect::<Vec<u32>>()
            .as_slice()
            .windows(2)
            .map(|x| x[0] < x[1])
            .filter(|&b| b)
            .count()
    } else { panic!("boah") }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_simple() {
        assert_eq!(simple_slide("src/one/test_input".to_string()), 7);
    }

    #[test]
    fn real_input_simple() {
        assert_eq!(simple_slide("src/one/input".to_string()), 1688);
    }

    #[test]
    fn sample_input_window() {
        assert_eq!(window_sum("src/one/test_input".to_string()), 5);
    }

    #[test]
    fn real_input_window() {
        assert_eq!(window_sum("src/one/input".to_string()), 1728);
    }

}