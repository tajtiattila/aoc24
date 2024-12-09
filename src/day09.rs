use std::collections::BTreeMap;
use std::iter;

pub fn run(input: &str) -> anyhow::Result<String> {
    let s1 = star1(input);
    let s2 = star2(input);
    Ok(format!("{s1} {s2}"))
}

fn star1(input: &str) -> usize {
    let m0 = diskmap(input)
        .flat_map(|(v, l)| iter::repeat(v).take(l))
        .collect::<Vec<_>>();

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

fn star2(input: &str) -> usize {
    let (mut freemap, mut files, _) = diskmap(input).fold(
        (BTreeMap::new(), vec![], 0),
        |(mut freemap, mut files, position), (id, len)| {
            if id == FREE_SPACE {
                freemap.insert(position, len);
            } else {
                files.push((position, id, len));
            }
            (freemap, files, position + len)
        },
    );
    for (pos, _, len) in files.iter_mut().rev() {
        if let Some((&new_pos, &free_len)) = freemap.range(..*pos).find(|(_, &l)| l >= *len) {
            freemap.remove(&new_pos);
            *pos = new_pos;
            if free_len > *len {
                freemap.insert(new_pos + *len, free_len - *len);
            }
        }
    }
    files
        .iter()
        .map(|(pos, id, len)| {
            let m = pos * len + (len * (len - 1)) / 2;
            m * (*id as usize)
        })
        .sum()
}

const FREE_SPACE: u32 = u32::MAX;

fn diskmap(input: &str) -> impl Iterator<Item = (u32, usize)> + use<'_> {
    struct State {
        file_id: u32,
        is_file: bool,
    }
    input.chars().scan(
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "2333133121414131402";
        assert_eq!(star1(input), 1928);
        assert_eq!(star2(input), 2858);
    }
}
