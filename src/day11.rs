use std::collections::HashMap;
use std::iter;

pub fn run(input: &str) -> anyhow::Result<String> {
    let stones = parse_stones(input)?;
    let s1 = blink_stones(&stones, 25);
    let s2 = blink_stones(&stones, 75);
    Ok(format!("{s1} {s2}"))
}

fn parse_stones(input: &str) -> anyhow::Result<Vec<u64>> {
    Ok(input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?)
}

fn blink_cached(m: &mut HashMap<(u64, usize), usize>, stone: u64, count: usize) -> usize {
    if count == 0 {
        return 1;
    }
    if let Some(n) = m.get(&(stone, count)) {
        return *n;
    }

    let n = if stone == 0 {
        blink_cached(m, 1, count - 1)
    } else if let Some((l, r)) = split_stone(stone) {
        blink_cached(m, l, count - 1) + blink_cached(m, r, count - 1)
    } else {
        blink_cached(m, stone * 2024, count - 1)
    };
    m.insert((stone, count), n);
    n
}

fn blink_stones(stones: &[u64], blink: usize) -> usize {
    let mut m = HashMap::new();
    stones
        .iter()
        .map(|stone| blink_cached(&mut m, *stone, blink))
        .sum()
}

fn split_stone(stone: u64) -> Option<(u64, u64)> {
    iter::successors(Some(1u64), |n| n.checked_mul(10))
        .map(|n| (n * 10, n * n * 10, n * n * 100))
        .take_while(|(_, lo, _)| *lo <= stone)
        .find_map(|(divmod, lo, hi)| {
            (lo..hi)
                .contains(&stone)
                .then(|| (stone / divmod, stone % divmod))
        })
}

#[cfg(test)]
mod test {
    use super::*;

    fn blink(stone: u64, count: usize) -> usize {
        blink_cached(&mut HashMap::new(), stone, count)
    }

    #[test]
    fn it_works() {
        assert_eq!(blink(0, 1), 1); // 1
        assert_eq!(blink(1, 1), 1); // 2024
        assert_eq!(blink(10, 1), 2); // 1 0
        assert_eq!(blink(99, 1), 2); // 9 9
        assert_eq!(blink(999, 1), 1); // 2021976
        let stones = parse_stones("0 1 10 99 999").unwrap();
        // blink: 1 2024 1 0 9 9 2021976
        assert_eq!(blink_stones(&stones, 1), 7);
    }

    #[test]
    fn sample2() {
        let stones = parse_stones("125 17").unwrap();
        assert_eq!(blink_stones(&stones, 6), 22);
        assert_eq!(blink_stones(&stones, 25), 55312);
    }
}
