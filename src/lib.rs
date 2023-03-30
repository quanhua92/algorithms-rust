pub mod data_structures;
pub mod search;
pub mod sort;

#[cfg(test)]
mod tests {
    use super::search;
    #[test]
    fn linear_search() {
        assert_eq!(search::linear_search(&5, &[1, 2, 3, 4, 5, 6]), Some(4));
    }
}
