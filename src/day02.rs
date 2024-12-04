use crate::util;

pub fn run(input: &str) -> anyhow::Result<String> {
    let s1 = count_safe(input);
    let s2 = count_safe_dampened(input);
    Ok(format!("{s1} {s2}"))
}

fn count_safe(input: &str) -> usize {
    input.lines().filter(|l| line_safe(l)).count()
}

fn line_levels(line: &str) -> impl Iterator<Item = i64> + use<'_> {
    line.split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
}

fn line_safe(line: &str) -> bool {
    levels_safe(line_levels(line))
}

fn levels_safe(levels: impl Iterator<Item = i64>) -> bool {
    let mut deltas = levels
        .scan(0, |state, x| {
            let d = x - *state;
            *state = x;
            Some(d)
        })
        .skip(1);

    let first = deltas.next().unwrap();
    if first == 0 || !(1..=3).contains(&first.abs()) {
        return false;
    }
    let ok_range = if first < 0 { -3..=-1 } else { 1..=3 };
    deltas.all(|x| ok_range.contains(&x))
}

fn count_safe_dampened(input: &str) -> usize {
    input.lines().filter(|l| line_safe_dampened(l)).count()
}

fn line_safe_dampened(line: &str) -> bool {
    let levels: Vec<_> = line_levels(line).collect();
    for skip in 0..levels.len() {
        if levels_safe(util::skip_nth(levels.iter().copied(), skip)) {
            return true;
        }
    }
    false
}
