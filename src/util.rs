pub fn skip_nth<T>(it: impl Iterator<Item = T>, skip: usize) -> impl Iterator<Item = T> {
    it.enumerate()
        .filter_map(move |(n, x)| (n != skip).then_some(x))
}
