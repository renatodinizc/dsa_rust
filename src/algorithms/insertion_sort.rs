pub fn sort<T: PartialOrd + Clone>(mut numbers: Vec<T>) -> Vec<T> {
    for i in 1..numbers.len() {
        let temp = numbers[i].clone();

        for j in (0..i).rev() {
            if numbers[j] > temp {
                numbers.swap(j, j + 1);
            } else {
                break;
            }
        }
    }
    numbers
}

#[cfg(test)]
mod tests {
    use super::sort;

    #[test]
    fn assert_insertion_sort_with_u32() {
        let numbers = vec![123u32, 45, 3, 28, 74, 19123, 28, 28, 1];

        let result = sort(numbers);

        assert_eq!(result, [1, 3, 28, 28, 28, 45, 74, 123, 19123]);
    }

    #[test]
    fn assert_insertion_sort_with_f64() {
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
    fn assert_insertion_sort_with_str() {
        let numbers = vec!["grapes", "potatoes", "apples", "pineapples", "watermelons"];

        let result = sort(numbers);

        assert_eq!(
            result,
            ["apples", "grapes", "pineapples", "potatoes", "watermelons"]
        );
    }

    #[test]
    fn assert_insertion_sort_with_string() {
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
}
