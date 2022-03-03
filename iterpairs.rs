/// Given an iterator producing items, return an iterator
/// producing pairs of items.
///
/// If the input iterator produces an odd number of items,
/// the resulting iterator will ignore the last item.
pub fn iter_pairs<T, I: Iterator<Item = T>>(mut iter: I) -> impl Iterator<Item = (T, T)> {
    std::iter::from_fn(move || {
        let x1 = iter.next()?;
        let x2 = iter.next()?;
        Some((x1, x2))
    })
}

#[test]
fn test_iter_pairs() {
    let v = vec![1, 2, 3, 4];
    let mut iter = iter_pairs(v.into_iter());
    assert_eq!(iter.next(), Some((1, 2)));
    assert_eq!(iter.next(), Some((3, 4)));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_pairs_odd() {
    let v = vec![1];
    let mut iter = iter_pairs(v.into_iter());
    assert_eq!(None, iter.next());
}
