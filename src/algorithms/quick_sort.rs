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
    use super::{partition, sort};

    #[test]
    fn assert_quick_sort_with_u32() {
        let numbers = vec![123u32, 45, 3, 28, 74, 19123, 28, 28, 1];

        let result = sort(numbers);

        assert_eq!(result, [1, 3, 28, 28, 28, 45, 74, 123, 19123]);
    }

    #[test]
    fn assert_quick_sort_with_f64() {
        let numbers = vec![
            123.4, 45f64, 3.1, 28.1, 28.02, 19123.97, 28.01, 28.001, 1f64,
        ];

        let result = sort(numbers);

        assert_eq!(
            result,
            [1f64, 3.1, 28.001, 28.01, 28.02, 28.1, 45f64, 123.4, 19123.97]
        );
    }

    #[test]
    fn assert_quick_sort_with_str() {
        let numbers = vec!["grapes", "potatoes", "apples", "pineapples", "watermelons"];

        let result = sort(numbers);

        assert_eq!(
            result,
            ["apples", "grapes", "pineapples", "potatoes", "watermelons"]
        );
    }

    #[test]
    fn assert_quick_sort_with_string() {
        let numbers = vec![
            "grapes".to_string(),
            "potatoes".to_string(),
            "apples".to_string(),
            "pineapples".to_string(),
            "watermelons".to_string(),
        ];

        let result = sort(numbers);

        assert_eq!(
            result,
            [
                "apples".to_string(),
                "grapes".to_string(),
                "pineapples".to_string(),
                "potatoes".to_string(),
                "watermelons".to_string()
            ]
        );
    }

    #[test]
    fn assert_partition_with_u32() {
        let mut numbers = vec![0u32, 5, 2, 1, 6, 3];

        let result = partition(&mut numbers);
        assert_eq!(result, 3);
    }

    #[test]
    fn assert_partition_with_f64() {
        let mut numbers = vec![0f64, 5.1, 2.001, 1.5, 6.3, 3f64];

        let result = partition(&mut numbers);
        assert_eq!(result, 3);
    }

    #[test]
    fn assert_partition_with_str() {
        let mut numbers = vec!["carrot", "apple", "grape", "banana", "zucchini", "lemon"];

        let result = partition(&mut numbers);
        assert_eq!(result, 4);
    }

    #[test]
    fn assert_partition_with_string() {
        let mut numbers = vec![
            "carrot".to_string(),
            "apple".to_string(),
            "grape".to_string(),
            "banana".to_string(),
            "zucchini".to_string(),
            "lemon".to_string(),
        ];

        let result = partition(&mut numbers);

        assert_eq!(result, 4);
    }
}
