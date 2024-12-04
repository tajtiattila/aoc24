pub fn run(input: &str) -> anyhow::Result<String> {
    let s1 = count_safe(input);
    Ok(format!("{s1}"))
}

fn count_safe(input: &str) -> usize {
    input.lines().filter(|l| line_safe(l)).count()
}

fn line_safe(line: &str) -> bool {
    let mut deltas = line
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
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
    let ok_range = if first < 0 {
        -3..=-1
    } else {
        1..=3
    };
    deltas.all(|x| ok_range.contains(&x))
}
