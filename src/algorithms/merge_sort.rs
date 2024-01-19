pub fn sort<T: PartialOrd + Clone>(numbers: Vec<T>) -> Vec<T> {
    if numbers.len() <= 1 {
        return numbers;
    }

    let half = numbers.len() / 2;

    let left_half = &numbers[0..half];
    let right_half = &numbers[half..];

    let sorted_left = sort(left_half.to_vec());
    let sorted_right = sort(right_half.to_vec());

    merge(sorted_left, sorted_right)
}

fn merge<T: PartialOrd + Clone>(left_vec: Vec<T>, right_vec: Vec<T>) -> Vec<T> {
    let mut sorted: Vec<T> = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < left_vec.len() && j < right_vec.len() {
        if left_vec[i] <= right_vec[j] {
            sorted.push(left_vec[i].clone());
            i += 1;
        } else {
            sorted.push(right_vec[j].clone());
            j += 1;
        }
    }

    sorted.extend_from_slice(&left_vec[i..]);
    sorted.extend_from_slice(&right_vec[j..]);

    sorted
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
    fn sort_already_sorted() {
        let input = vec![1, 2, 3, 4, 5];
        assert_eq!(sort(input.clone()), input);
    }
}
