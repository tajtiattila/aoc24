use std::iter;

pub fn run(input: &str) -> anyhow::Result<String> {
    let s1 = star1(input);
    let s2 = "";
    Ok(format!("{s1} {s2}"))
}

fn star1(input: &str) -> usize {
    let m0 = diskmap(input);

    let it_free = m0
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| (c == FREE_SPACE).then_some(i));
    let it_file = m0
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(i, &c)| (c != FREE_SPACE).then_some(i));

    let m1 = iter::zip(it_free, it_file).take_while(|(x, y)| x < y).fold(
        m0.clone(),
        |mut acc, (x, y)| {
            acc.swap(x, y);
            acc
        },
    );
    m1.iter()
        .enumerate()
        .filter_map(|(i, &c)| (c != FREE_SPACE).then_some(i * (c as usize)))
        .sum()
}

const FREE_SPACE: u32 = u32::MAX;

fn diskmap(input: &str) -> Vec<u32> {
    struct State {
        file_id: u32,
        is_file: bool,
    }
    input
        .chars()
        .scan(
            State {
                file_id: 0,
                is_file: true,
            },
            |state, c| {
                let l = if c.is_ascii_digit() {
                    (c as usize) - ('0' as usize)
                } else {
                    return None;
                };
                let v;
                if state.is_file {
                    v = state.file_id;
                    state.file_id += 1;
                } else {
                    v = FREE_SPACE;
                }
                state.is_file = !state.is_file;
                Some((v, l))
            },
        )
        .flat_map(|(v, l)| iter::repeat(v).take(l))
        .collect()
}
