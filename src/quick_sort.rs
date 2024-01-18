pub fn partition(numbers: &mut [u32]) -> &[u32] {
    let pivot_index = numbers.len() - 1usize;
    let pivot_value = numbers[pivot_index];

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
