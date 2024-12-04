use std::collections::HashMap;
use std::iter::zip;

pub fn run(input: &str) -> anyhow::Result<String> {
    Ok(format!("{} {}", star1(input), star2(input)))
}

fn get_lists(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut v0: Vec<usize> = vec![];
    let mut v1: Vec<usize> = vec![];
    for (x, y) in input.lines().filter_map(|l| l.split_once(' ')) {
        v0.push(x.parse().unwrap());
        v1.push(y.trim().parse().unwrap());
    }
    v0.sort();
    v1.sort();
    (v0, v1)
}

fn star1(input: &str) -> usize {
    let (v0, v1) = get_lists(input);
    zip(v0, v1)
        .map(|(l, r)| if l < r { r - l } else { l - r })
        .sum()
}

fn star2(input: &str) -> usize {
    let (v0, v1) = get_lists(input);

    let mut occurrences = HashMap::new();
    for y in v1 {
        occurrences.entry(y).and_modify(|n| *n += 1).or_insert(1);
    }
    v0.into_iter()
        .map(|x| x * occurrences.get(&x).unwrap_or(&0))
        .sum()
}
