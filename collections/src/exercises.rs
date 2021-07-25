use std::collections::HashMap;
use std::convert::TryInto;

pub fn stats(mut v: Vec<i32>) -> Stats {
    Stats {
        mean: mean(&v),
        median: median(&mut v),
        mode: mode(&v),
    }
}

fn mean(v: &Vec<i32>) -> i32 {
    let sum: i32 = v.into_iter().sum();
    let count = v.into_iter().count();
    sum.checked_div(count.try_into().unwrap_or(0)).unwrap_or(0)
}

fn median(v: &mut Vec<i32>) -> i32 {
    v.sort();
    let count = v.into_iter().count() - 1;
    *v.get(count.try_into().unwrap_or(0) / 2).unwrap_or(&0)
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut hm = HashMap::new();
    for i in v {
        let count = hm.entry(*i).or_insert(0);
        *count += 1
    }
    *hm.iter_mut()
        .max_by_key(|(_, &mut count)| count)
        .map(|(mode, _)| mode)
        .unwrap_or(&mut 0)
}

#[derive(Debug)]
pub struct Stats {
    mean: i32,
    median: i32,
    mode: i32,
}
