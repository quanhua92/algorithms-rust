use std::fmt::Display;

/// Sort a list using quick sort algorithm.
///
/// # Examples
/// ```
/// use algorithms::sort::quick_sort;
/// let mut arr = [3, 2, 1];
/// quick_sort(&mut arr);
/// assert_eq!(arr, [1, 2, 3]);
/// ```
pub fn quick_sort<T: PartialOrd + Copy + Clone + Display>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    _quick_sort(arr, 0, arr.len() - 1);
}

pub fn _quick_sort<T: PartialOrd + Display>(arr: &mut [T], low: usize, high: usize) {
    if low < high {
        let pivot = partition(arr, low, high);
        if pivot > 0 {
            _quick_sort(arr, low, pivot - 1);
        }
        _quick_sort(arr, pivot + 1, high);
    }
}

fn partition<T: PartialOrd>(arr: &mut [T], low: usize, high: usize) -> usize {
    let mut i = low;

    for j in low..=high - 1 {
        if arr[j] <= arr[high] {
            arr.swap(i as usize, j as usize);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sort_ascending() {
        let mut arr = [1, 2, 3];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }
    #[test]
    fn sort_descending() {
        let mut arr = [3, 2, 1];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }
    #[test]
    fn sort_descending_float() {
        let mut arr = [3.0, 2.0, 1.0];
        quick_sort(&mut arr);
        assert_eq!(arr, [1.0, 2.0, 3.0]);
    }
    #[test]
    fn sort_empty() {
        let mut arr: [i32; 0] = [];
        quick_sort(&mut arr);
        assert_eq!(arr, []);
    }
    #[test]
    fn sort_random() {
        let mut arr = [5, 1, 2, 3, 4, 6];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
