pub fn sort<T: PartialOrd + Clone>(mut numbers: Vec<T>) -> Vec<T> {
    if numbers.len() <= 1 {
        return numbers;
    }

    let pivot_index = partition(&mut numbers);

    let mut l_part = sort(numbers[..pivot_index].to_vec());
    let r_part = sort(numbers[pivot_index + 1..].to_vec());

    l_part.push(numbers[pivot_index].clone());
    l_part.extend(r_part);

    l_part
}

fn partition<T: PartialOrd + Clone>(numbers: &mut [T]) -> usize {
    let pivot_index = numbers.len() - 1;
    let pivot_value = numbers[pivot_index].clone();

    let mut l_pointer = 0;
    let mut r_pointer = pivot_index;

    while l_pointer < r_pointer {
        while l_pointer < r_pointer && numbers[l_pointer] < pivot_value {
            l_pointer += 1;
        }

        while l_pointer < r_pointer && numbers[r_pointer] >= pivot_value {
            r_pointer -= 1;
        }

        if l_pointer < r_pointer {
            numbers.swap(l_pointer, r_pointer);
        }
    }

    numbers.swap(l_pointer, pivot_index);

    l_pointer
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
