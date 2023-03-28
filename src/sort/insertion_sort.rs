/// Sort a list using the insertion sort algorithm.
///
/// # Examples
/// ```
/// use algorithms::sort::insertion_sort;
/// let mut arr = [3, 2, 1];
/// insertion_sort(&mut arr);
/// assert_eq!(arr, [1, 2, 3]);
/// ```
pub fn insertion_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    let len = arr.len();

    for i in 1..len {
        let mut pos = i;
        let temp = arr[i];
        while pos > 0 && temp < arr[pos - 1] {
            arr[pos] = arr[pos - 1];
            pos -= 1;
        }
        arr[pos] = temp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sort_ascending() {
        let mut arr = [1, 2, 3];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }
    #[test]
    fn sort_descending() {
        let mut arr = [3, 2, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }
    #[test]
    fn sort_descending_float() {
        let mut arr = [3.0, 2.0, 1.0];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1.0, 2.0, 3.0]);
    }
    #[test]
    fn sort_empty() {
        let mut arr: [i32; 0] = [];
        insertion_sort(&mut arr);
        assert_eq!(arr, []);
    }
}
