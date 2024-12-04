// Extrapolate the nth value from the values in it,
// assuming the values in the sequence grow polynomially.
#[allow(unused)]
pub fn nth(mut it: impl Iterator<Item = usize>, n: usize) -> Option<usize> {
    let mut v = vec![];

    loop {
        let mut x1 = it.next()?;

        if n == v.len() {
            return Some(x1);
        }

        for x0 in &mut v {
            let d = x1.checked_sub(*x0)?;
            *x0 = x1;

            x1 = d;
        }

        v.push(x1);

        if x1 == 0 && v.len() > 1 {
            break;
        }
    }

    let nfwd = n - v.len() + 1;

    Some(from_diffv(&v, nfwd))
}

fn from_diffv(v: &[usize], n: usize) -> usize {
    let mut v = v;

    // Remove unnecessary zeros at the end to avoid overflows.
    while !v.is_empty() && *v.last().unwrap() == 0 {
        v = &v[..v.len() - 1];
    }

    // Similarly, calculate multipliers lazily.
    // ms = [1/0!, n/1!, n*(n+1)/2!, n*(n+1)*(n+2)/3!, ... n*(n+1)*..*(n+i-1)/i!
    let ms = std::iter::once(1).chain((0..).scan(1, |state, i| {
        *state = *state * (n + i) / (i + 1);
        Some(*state)
    }));

    std::iter::zip(v, ms).map(|(&x, m)| x * m).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let s1 = [26, 216, 588, 1142, 1878];
        // s1[30] == 84896; s1[90] == 746036
        assert_eq!(nth(s1.into_iter(), 2), Some(588));
        assert_eq!(nth(s1.into_iter(), 30), Some(84896));
        assert_eq!(nth(s1.into_iter(), 90), Some(746036));

        let s2 = [3797, 34009, 94353, 184829, 305437, 456177];
        assert_eq!(nth(s2.into_iter(), 202300), Some(616583483179597));
    }
}
