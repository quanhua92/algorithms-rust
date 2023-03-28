/// Sort a list using the selection sort algorithm.
///
/// # Examples
/// ```
/// use algorithms::sort::selection_sort;
/// let mut arr = [3, 2, 1];
/// selection_sort(&mut arr);
/// assert_eq!(arr, [1, 2, 3]);
/// ```
pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        let mut lowest_index = i;
        for j in i + 1..len {
            if arr[j] < arr[lowest_index] {
                lowest_index = j;
            }
        }
        if lowest_index != i {
            arr.swap(i, lowest_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sort_ascending() {
        let mut arr = [1, 2, 3];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }
    #[test]
    fn sort_descending() {
        let mut arr = [3, 2, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }
    #[test]
    fn sort_descending_float() {
        let mut arr = [3.0, 2.0, 1.0];
        selection_sort(&mut arr);
        assert_eq!(arr, [1.0, 2.0, 3.0]);
    }
    #[test]
    fn sort_empty() {
        let mut arr: [i32; 0] = [];
        selection_sort(&mut arr);
        assert_eq!(arr, []);
    }
    #[test]
    fn sort_random() {
        let mut arr = [5, 1, 2, 3, 4, 6];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
