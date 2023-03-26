pub fn linear_search<T: PartialEq>(value: &T, arr: &[T]) -> Option<usize> {
    for (idx, elem) in arr.iter().enumerate() {
        if elem == value {
            return Some(idx);
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
        assert_eq!(linear_search(&5, &a), Some(4));
        assert_eq!(linear_search(&-1, &a), None);
    }

    #[test]
    fn search_strings() {
        assert_eq!(linear_search(&"a", &["a", "b", "c", "d"]), Some(0));
        assert_eq!(linear_search(&"", &["a", "b", "c", "d"]), None);
    }

    #[test]
    fn search_not_found() {
        assert_eq!(linear_search(&10, &[1, 2, 3, 4, 5]), None);
    }

    #[test]
    fn search_empty() {
        let b = vec![];
        assert_eq!(linear_search(&5, &b), None);
        assert_eq!(linear_search(&-1, &b), None);
    }
}
