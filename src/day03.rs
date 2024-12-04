use regex::Regex;

pub fn run(input: &str) -> anyhow::Result<String> {
    let s1 = star1(input);
    let s2 = "";
    Ok(format!("{s1} {s2}"))
}

fn star1(input: &str) -> usize {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|captures| captures.extract())
        .filter_map(|(_, [x, y])| Some(x.parse::<usize>().ok()? * y.parse::<usize>().ok()?))
        .sum()
}
