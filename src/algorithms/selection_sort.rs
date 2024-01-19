pub fn sort<T: PartialOrd>(mut numbers: Vec<T>) -> Vec<T> {
    for i in 0..numbers.len() {
        let mut lowest_number_index = i;

        for j in i..numbers.len() {
            if numbers[j] < numbers[lowest_number_index] {
                lowest_number_index = j;
            }
        }
        numbers.swap(i, lowest_number_index);
    }
    numbers
}

#[cfg(test)]
mod tests {
    use super::sort;

    #[test]
    fn sort_numbers() {
        let input = vec![4, 2, 3, 1, 5];
        assert_eq!(sort(input), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn sort_empty_vec() {
        let empty_vec: Vec<i32> = vec![];
        assert_eq!(sort(empty_vec), vec![]);
    }

    #[test]
    fn sort_single_element_vec() {
        let single_element = vec![1];
        assert_eq!(sort(single_element), vec![1]);
    }

    #[test]
    fn sort_characters() {
        let chars = vec!['c', 'a', 'b', 'e', 'd'];
        assert_eq!(sort(chars), vec!['a', 'b', 'c', 'd', 'e']);
    }

    #[test]
    fn sort_strs() {
        let strs = vec!["beta", "alpha", "gamma"];
        assert_eq!(sort(strs), vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn sort_strings() {
        let strings = vec!["beta".to_string(), "alpha".to_string(), "gamma".to_string()];
        assert_eq!(sort(strings), vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn sort_already_selection_sorted() {
        let input = vec![1, 2, 3, 4, 5];
        assert_eq!(sort(input.clone()), input);
    }
}
