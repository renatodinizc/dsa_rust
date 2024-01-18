pub fn partition<T: PartialOrd + Clone>(numbers: &mut [T]) -> &[T] {
    let pivot_index = numbers.len() - 1usize;
    let pivot_value = numbers[pivot_index].clone();

    let mut l = 0;
    let mut r = pivot_index - 1;

    loop {
        while numbers[l] < pivot_value {
            l += 1;
        }

        while numbers[r] > pivot_value {
            r -= 1;
        }

        if l >= r {
            break;
        } else {
            numbers.swap(l, r);
        }
    }

    numbers.swap(l, pivot_index);

    numbers
}

#[test]
fn assert_partition_with_u32() {
    let mut numbers = vec![0u32, 5, 2, 1, 6, 3];

    let result = partition(&mut numbers);
    assert_eq!(result, [0, 1, 2, 3, 6, 5]);
}

#[test]
fn assert_partition_with_f64() {
    let mut numbers = vec![0f64, 5.1, 2.001, 1.5, 6.3, 3f64];

    let result = partition(&mut numbers);
    assert_eq!(result, [0f64, 1.5, 2.001, 3f64, 6.3, 5.1]);
}

#[test]
fn assert_partition_with_str() {
    let mut numbers = vec!["carrot", "apple", "grape", "banana", "zucchini", "lemon"];

    let result = partition(&mut numbers);
    assert_eq!(
        result,
        ["carrot", "apple", "grape", "banana", "lemon", "zucchini"]
    );
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

    assert_eq!(
        result,
        [
            "carrot".to_string(),
            "apple".to_string(),
            "grape".to_string(),
            "banana".to_string(),
            "lemon".to_string(),
            "zucchini".to_string(),
        ]
    );
}
