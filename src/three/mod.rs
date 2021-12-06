use std::ops::Deref;
use std::str::FromStr;
use bitvec::vec::BitVec;

use itertools::Itertools;
use nalgebra::{DMatrix, Matrix5xX, MatrixXx5, SimdBool};
use num_integer::Integer;

use crate::little_helpers;
use crate::little_helpers::read_lines;

fn power_consumtion(input: &str) -> usize {
    let mat = parse(input);

    let more_than_half: u32 = dbg!(mat.ncols().div_ceil(&2) as u32);

    let gamma: usize = mat.row_iter().fold(0, |acc, x| {
        if dbg!(x.sum()) >= more_than_half  {
            dbg!(acc * 2 + 1)
        } else {
            dbg!(acc * 2)
        }
    });

    let epsilon: usize = mat.row_iter().fold(0, |acc, x| {
        if x.sum() >= more_than_half {
            acc * 2
        } else {
            acc * 2 + 1
        }
    });

    dbg!(gamma);
    dbg!(epsilon);

    (gamma * epsilon) as usize
}

fn life_support_rating(input: &str) -> usize {
    let mut mat = parse(input);
    reduce_matrix(0, mat.nrows(), &mat, true) *
    reduce_matrix(0, mat.nrows(), &mat, false)
}

fn reduce_matrix(position: usize, max: usize, input: &DMatrix<u32>, most: bool) -> usize {
    if position == max || input.ncols() == 1 {
        return dbg!(input).column(0).fold(0, |acc, x| dbg!(acc * 2 + x as usize) as usize);
    }

    let more_than_half = dbg!(input.ncols().div_ceil(&2) as u32);
    let most_common: u32 = if dbg!(input.row(position).sum()) >= more_than_half {
        if most { 1 } else { 0 }
    } else {
        if most { 0 } else { 1 }
    };

    let filter = input.column_iter()
        .filter(|x| *(x.get(position).unwrap()) == most_common);

    let len = filter.clone().into_iter().count();
    let mat: DMatrix<u32> = DMatrix::from_iterator(dbg!(input.nrows()), len, filter.into_iter()
        .flat_map(|x| x.iter().map(|&x| x.clone()).collect::<Vec<u32>>()));

    reduce_matrix(position+1, max, &mat, most)
}

fn parse(input: &str) -> DMatrix<u32> {
    let width = input.lines().last().unwrap().len();
    let len = input.lines().count();

    DMatrix::from_vec(width, len, input.lines()
        .into_iter()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()))
        .flat_map(|x| x.into_iter())
        .collect::<Vec<u32>>())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_simple() {
        assert_eq!(power_consumtion(include_str!("./test_input")), 198);
    }

    #[test]
    fn input_simple() {
        assert_eq!(power_consumtion(include_str!("./input")), 2250414);
    }

    #[test]
    fn sample_input() {
        assert_eq!(life_support_rating(include_str!("./test_input")), 230);
    }

    #[test]
    fn input() {
        assert_eq!(life_support_rating(include_str!("./input")), 230);
    }
}