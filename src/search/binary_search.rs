use core::cmp::Ord;
/// Searches for the first occurence of an element in an ordered list (sorted in ascending order) using the binary search algorithm.
/// Returns the position with [`Option<usize>`]
/// or [`None`] if can not find the element.
///
/// # Examples
/// ```
/// use algorithms::search::binary_search;
/// assert_eq!(binary_search(&"a", &["a", "b", "c", "d"]), Some(0));
/// assert_eq!(binary_search(&"", &["a", "b", "c", "d"]), None);
///
/// assert_eq!(binary_search(&5, &[1, 2, 3, 4, 5]), Some(4));
/// assert_eq!(binary_search(&-1, &[1, 2, 3, 4, 5]), None);
/// assert_eq!(binary_search(&-1, &[]), None);
/// ```
pub fn binary_search<T: Ord>(value: &T, arr: &[T]) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = (right + left) / 2;
        match arr[mid].cmp(&value) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => {
                if mid == 0 {
                    return None;
                }
                right = mid - 1;
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_ints() {
        let a = vec![1, 2, 3, 4, 5];
        assert_eq!(binary_search(&5, &a), Some(4));
        assert_eq!(binary_search(&-1, &a), None);
    }

    #[test]
    fn search_strings() {
        assert_eq!(binary_search(&"a", &["a", "b", "c", "d"]), Some(0));
        assert_eq!(binary_search(&"", &["a", "b", "c", "d"]), None);
    }

    #[test]
    fn search_not_found() {
        assert_eq!(binary_search(&10, &[1, 2, 3, 4, 5]), None);
    }

    #[test]
    fn search_empty() {
        let b = vec![];
        assert_eq!(binary_search(&5, &b), None);
        assert_eq!(binary_search(&-1, &b), None);
    }
}
