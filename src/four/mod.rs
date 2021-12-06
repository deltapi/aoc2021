use std::io::Lines;

use itertools::Itertools;
use nalgebra::Matrix5;

fn calculate_winning_board_score(input: &str) -> usize {
    let (draws, mut boards) = parse(input);

    for draw in draws {
        for mut board in boards.iter_mut() {
            apply_draw(draw, board);

            if has_won(&board) {
                dbg!(&board);
                return board.into_iter()
                    .filter(|(x, t)| !t)
                    .map(|(x, t)| *x)
                    .sum::<usize>() * draw;
            }
        }
    }
    0
}

fn apply_draw(draw: usize, mut board: &mut Matrix5<(usize, bool)>) {
    *board = Matrix5::from_iterator(board.iter_mut().map(|&mut (x, mut t)| {
        if x == draw {
            (x.clone(), true)
        } else {
            (x.clone(), t.clone())
        }
    }));
}

fn calculate_loosing_board_score(input: &str) -> usize {
    let (draws, mut boards) = parse(input);

    for draw in draws {
        if boards.iter().filter(|x| !has_won(x)).count() > 1 {

        boards = boards.iter_mut().filter(|x| !has_won(x))
            .map(|x| {
                apply_draw(draw, x);
                x.clone()
            }).collect();
        }
        if boards.iter().filter(|x| !has_won(x)).count() == 1 {
            boards = boards.iter_mut().map(|x| {
                apply_draw(draw, x);
                x.clone()
            }).collect();

            let score =  boards.iter().last().unwrap().into_iter()
                .filter(|(x, t)| !t)
                .map(|(x, t)| *x)
                .sum::<usize>() * dbg!(draw);

            if boards.iter().filter(|x| !has_won(x)).count() == 0 {
                return score;
            }
        }
    }
    0
}

fn has_won(board: &Matrix5<(usize, bool)>) -> bool {
    board.row_iter()
        .filter(|&x| x.iter().filter(|&x| x.1).count() == 5)
        .any(|_| true)
        ||
        board.column_iter()
            .filter(|&x| x.iter().filter(|&x| x.1).count() == 5)
            .any(|_| true)
}

fn parse(input: &str) -> (Vec<usize>, Vec<Matrix5<(usize, bool)>>) {
    let mut iter = input.lines();
    let draws = iter.clone()
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<usize>().expect("Whaaaaaaaaaaaaaaaaa"))
        .collect::<Vec<usize>>();


    let bla = iter.skip(1)
        .chunks(6)
        .into_iter()
        .map(|chunk| Matrix5::from_vec(chunk.skip(1)
            .map(|line| parse_line(line))
            .flat_map(|v| v.into_iter())
            .map(|bla| (bla, false))
            .collect::<Vec<(usize, bool)>>()))
        .collect::<Vec<Matrix5<(usize, bool)>>>();

    (draws, bla)
}

fn parse_line(input: &str) -> Vec<usize> {
    input.trim().split(' ')
        .filter(|&x| !x.is_empty())
        .map(|x| x.parse::<usize>().expect("Whooooo"))
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_simple() {
        assert_eq!(calculate_winning_board_score(include_str!("./test_input")), 4512);
    }

    #[test]
    fn input_simple() {
        assert_eq!(calculate_winning_board_score(include_str!("./input")), 82440);
    }

    #[test]
    fn sample_input() {
        assert_eq!(calculate_loosing_board_score(include_str!("./test_input")), 1924);
    }

    #[test]
    fn input() {
        assert_eq!(calculate_loosing_board_score(include_str!("./input")), 82440);
    }
}