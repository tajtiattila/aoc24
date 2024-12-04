use regex::Regex;

pub fn run(input: &str) -> anyhow::Result<String> {
    let s1 = star1(input);
    let s2 = star2(input);
    Ok(format!("{s1} {s2}"))
}

fn star1(input: &str) -> usize {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|captures| captures.extract())
        .filter_map(|(_, [x, y])| Some(x.parse::<usize>().ok()? * y.parse::<usize>().ok()?))
        .sum()
}

fn star2(input: &str) -> usize {
    let mut enable = true;
    let mut sum: usize = 0;
    let re = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    for caps in re.captures_iter(input) {
        match caps.get(0).map_or("", |m| m.as_str()) {
            "do()" => {
                enable = true;
            }
            "don't()" => {
                enable = false;
            }
            _ => {
                if enable {
                    let x = caps.get(1).and_then(|x| x.as_str().parse::<usize>().ok());
                    let y = caps.get(2).and_then(|x| x.as_str().parse::<usize>().ok());
                    if let (Some(x), Some(y)) = (x, y) {
                        sum += x * y;
                    }
                }
            }
        }
    }
    return sum;
}
