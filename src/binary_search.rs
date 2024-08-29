pub fn bin_search<T: Ord>(arr: &[T], val: T) -> Option<usize> {
    let mut l = 0;
    let mut r = arr.len() - 1;

    while l <= r {
        let m = (l + r) / 2;
        if arr[m] < val {
            l = m + 1;
        } else if arr[m] > val {
            r = m - 1;
        } else {
            return Some(m);
        }
    }
    None
}
