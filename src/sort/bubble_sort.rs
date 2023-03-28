/// Sort a list using the bubble sort algorithm.
///
/// # Examples
/// ```
/// use algorithms::sort::bubble_sort;
/// let mut arr = [3, 2, 1];
/// bubble_sort(&mut arr);
/// assert_eq!(arr, [1, 2, 3]);
/// ```
pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..(len - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sort_ascending() {
        let mut arr = [1, 2, 3];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }
    #[test]
    fn sort_descending() {
        let mut arr = [3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }
    #[test]
    fn sort_descending_float() {
        let mut arr = [3.0, 2.0, 1.0];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1.0, 2.0, 3.0]);
    }
    #[test]
    fn sort_empty() {
        let mut arr: [i32; 0] = [];
        bubble_sort(&mut arr);
        assert_eq!(arr, []);
    }
}
