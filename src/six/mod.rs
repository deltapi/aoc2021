use std::collections::HashMap;

use itertools::Itertools;

fn bla(input: &str, days: usize) -> usize {
    let mut vec = input.split(',')
        .into_iter()
        .map(|x| x.parse::<usize>().expect("Whaaaaaaaaaaaaaaaaa"))
        .collect();

    for i in (0..days).into_iter() {
        vec = tick(vec);
    }
    vec.len()
}

fn tick(input: Vec<usize>) -> Vec<usize> {
    let mut out = vec![];

    input.into_iter().for_each(|x|
        match x {
            0 => {
                out.push(8);
                out.push(6);
            }
            _ => out.push((x as isize - 1) as usize),
        }
    );
    out
}

fn fast(input: &str, days: usize) -> usize {
    let vec = input.split(',')
        .into_iter()
        .map(|x| x.parse::<usize>().expect("Whaaaaaaaaaaaaaaaaa"))
        .collect();

    tick_faster(vec, days)
}

fn tick_faster(input: Vec<usize>, days: usize) -> usize {
    let mut out = input.clone();

    let mut days_count = HashMap::new();

    let remaining = days % 7;

    let cycles = days / 7;

    for _ in 0..remaining {
        out = tick(out);
    }

    out.into_iter().for_each(|x| {
        match days_count.get_mut(&x) {
            None => {
                days_count.insert(x, 1);
            }
            Some(i) => {
                *i += 1;
            }
        }
    });

    dbg!(&days_count);

    for _ in 0..cycles {
        let mut tmp: HashMap<usize, isize> = HashMap::new();

        tmp.insert(0, days_count.get(&0).unwrap_or(&0)+ days_count.get(&7).unwrap_or(&0));
        tmp.insert(1, days_count.get(&1).unwrap_or(&0)+ days_count.get(&8).unwrap_or(&0));
        tmp.insert(2, days_count.get(&2).unwrap_or(&0)+ days_count.get(&0).unwrap_or(&0));
        tmp.insert(3, days_count.get(&3).unwrap_or(&0)+ days_count.get(&1).unwrap_or(&0));
        tmp.insert(4, days_count.get(&4).unwrap_or(&0)+ days_count.get(&2).unwrap_or(&0));
        tmp.insert(5, days_count.get(&5).unwrap_or(&0)+ days_count.get(&3).unwrap_or(&0));
        tmp.insert(6, days_count.get(&6).unwrap_or(&0)+ days_count.get(&4).unwrap_or(&0));
        tmp.insert(7, *days_count.get(&5).unwrap_or(&0));
        tmp.insert(8, *days_count.get(&6).unwrap_or(&0));

        days_count = tmp;

        dbg!(&days_count);
    }

    //    dbg!(&days_count);

    days_count.values().into_iter().map(|&i| i as usize).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_simple() {
        assert_eq!(fast("3,4,3,1,2", 18), 26);
    }

    #[test]
    fn sample_input_simple_2() {
        assert_eq!(fast("3,4,3,1,2", 80), 5934);
    }

    #[test]
    fn sample_input_simple_3() {
        assert_eq!(fast("3,4,3,1,2", 256), 26984457539);
    }

    #[test]
    fn input_simple() {
        assert_eq!(fast(include_str!("./input"), 256), 5934);
    }
}