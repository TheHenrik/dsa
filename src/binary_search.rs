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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n() {
        let arr: [usize; 1_000] = core::array::from_fn(|i| i * 2);
        assert_eq!(bin_search(&arr, 0), Some(0));
        assert_eq!(bin_search(&arr, 100), Some(50));
        assert_eq!(bin_search(&arr, 1), None);
        assert_eq!(bin_search(&arr, 10_000), None);
    }
}
