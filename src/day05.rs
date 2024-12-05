use anyhow::{anyhow, Result};

pub fn run(input: &str) -> Result<String> {
    let pj = parse_print_job(input)?;
    let s1 = star1(&pj);
    let s2 = "";
    Ok(format!("{s1} {s2}"))
}

struct PrintJob {
    ord: Vec<(i32,i32)>,
    upd: Vec<Vec<i32>>,
}

fn parse_print_job(input: &str) -> Result<PrintJob> {
    let mut pj = PrintJob{
        ord: vec![],
        upd: vec![],
    };

    let mut it = input.lines();

    while let Some(line) = it.next() {
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        let (x, y) = line.split_once('|').ok_or_else(|| anyhow!("invalid line"))?;
        pj.ord.push((x.parse()?,y.parse()?));
    }

    while let Some(line) = it.next() {
        pj.upd.push(line.trim().split(',').map(|x| x.parse::<i32>()).collect::<Result<Vec<_>, _>>()?);
    }

    Ok(pj)
}

fn star1(pj: &PrintJob) -> usize {
    let mut pages_after = vec![0u128; 100];
    for &(x, y) in pj.ord.iter() {
        pages_after[x as usize] |= 1u128<<y;
    }

    pj.upd.iter().filter_map(|pr| {
        let order_ok = pr.iter().scan(0u128, |state, n| {
            let pred = *state;
            *state |= 1u128<<n;
            Some((pred, n))
        }).all(|(preds, n)| pages_after[*n as usize] & preds == 0);
        order_ok.then_some(pr[pr.len()/2 as usize] as usize)
    }).sum()
}
