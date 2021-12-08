use std::collections::HashMap;
use itertools::Itertools;

fn bla(input: &str) -> usize {
    input.lines().into_iter().map(|s| s.split('|')
        .skip(1)
        .find(|x| true)
        .expect("Did not find second part")
        .trim()
        .split(' ')
        .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
        .count()
    ).sum()
}

fn blub(input: &str) -> usize {
    input.lines().into_iter().map(|s| parse_line(s))
        .into_iter()
        .sum::<usize>()
}

fn parse_line(line: &str) -> usize {
    let test_signals = line.clone().split('|')
        .take(1)
        .find(|_| true)
        .expect("could not parse test signals")
        .trim()
        .split(' ')
        .map(|s| s.chars().sorted().collect::<String>())
        .collect::<Vec<String>>();

    let one = dbg!(test_signals.iter().filter(|s| s.len() == 2).last().unwrap()).as_str();
    let four = dbg!(test_signals.iter().filter(|s| s.len() == 4).last().unwrap()).as_str();
    let seven = dbg!(test_signals.iter().filter(|s| s.len() == 3).last().unwrap()).as_str();
    let eight = dbg!(test_signals.iter().filter(|s| s.len() == 7).last().unwrap()).as_str();

    let zero = dbg!(test_signals.iter()
        .filter(|s| s.len() == 6
            && four.chars().into_iter().filter(|&c| !s.contains(c)).last().is_some()
            && one.chars().into_iter().filter(|&c| !s.contains(c)).last().is_none()
        )
        .last()
        .unwrap()).as_str();

    let nine = dbg!(test_signals.iter()
        .filter(|s| s.len() == 6 && four.chars().into_iter().filter(|&c| !s.contains(c)).last().is_none())
        .last()
        .unwrap()).as_str();

    let six = dbg!(test_signals.iter()
        .filter(|s| s.len() == 6
            && four.chars().into_iter().filter(|&c| !s.contains(c)).last().is_some()
            && one.chars().into_iter().filter(|&c| !s.contains(c)).last().is_some()
        )
        .last()
        .unwrap().as_str());

    let three = dbg!(test_signals.iter()
        .filter(|s| s.len() == 5 && seven.chars().into_iter().filter(|&c| !s.contains(c)).last().is_none())
        .last()
        .unwrap().as_str());

    let five = dbg!(test_signals.iter()
        .filter(|&s| s.len() == 5 && s.chars().into_iter().filter(|&c| !six.contains(c)).last().is_none())
        .last()
        .unwrap().as_str());

    let two = dbg!(test_signals.iter()
        .filter(|&s| s.len() == 5
            && s.chars().into_iter().filter(|&c| !six.contains(c)).last().is_some()
            && one.chars().into_iter().filter(|&c| !s.contains(c)).last().is_some()
        )
        .last()
        .unwrap().as_str());

    let values = HashMap::from([
        (zero, "0"),
        (one, "1"),
        (two, "2"),
        (three, "3"),
        (four, "4"),
        (five, "5"),
        (six, "6"),
        (seven, "7"),
        (eight, "8"),
        (nine, "9")
    ]);

    line.clone().split('|')
        .skip(1)
        .take(1)
        .find(|_| true)
        .expect("Could not parse signals")
        .split(' ')
        .filter(|&s| s != "")
        .map(|s| values.get(dbg!(s.chars().sorted().collect::<String>()).as_str())
            .expect("value not found")
            .to_string()
        )
        .collect::<String>()
        .parse::<usize>()
        .expect("not a usize")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_simple() {
        assert_eq!(bla(include_str!("./test_input")), 26);
    }

    #[test]
    fn input_simple() {
        assert_eq!(bla(include_str!("./input")), 37);
    }

    #[test]
    fn sample_input() {
        assert_eq!(blub(include_str!("./test_input")), 61229);
    }

    #[test]
    fn input() {
        assert_eq!(blub(include_str!("./input")), 1084606);
    }
}