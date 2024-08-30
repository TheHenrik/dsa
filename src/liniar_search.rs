pub fn lin_search<T: Eq>(arr: &[T], val: T) -> Option<usize> {
    for (i, v) in arr.iter().enumerate() {
        if *v == val {
            return Some(i);
        }
    }
    None
}

pub fn count<T: Eq>(arr: &[T], val: T) -> Option<usize> {
    let mut c = 0;

    for v in arr {
        if *v == val {
            c += 1;
        }
    }

    if c == 0 {
        return None;
    }
    Some(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n() {
        let arr: [usize; 1_000] = core::array::from_fn(|i| i * 2);
        assert_eq!(lin_search(&arr, 0), Some(0));
        assert_eq!(lin_search(&arr, 100), Some(50));
        assert_eq!(lin_search(&arr, 1), None);
        assert_eq!(lin_search(&arr, 10_000), None);
    }

    #[test]
    fn m() {
        let arr: [usize; 1_000] = core::array::from_fn(|i| i * 2 * (i % 2));
        assert_eq!(count(&arr, 2), Some(1)); 
        assert_eq!(count(&arr, 0), Some(500));
        assert_eq!(count(&arr, 1), None);
    }
}
