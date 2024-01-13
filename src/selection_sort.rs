pub fn sort(mut numbers: Vec<u32>) {
    let mut lowest_number_index = 0;
    let mut i = 0;

    while i < numbers.len() {
        let mut j = i;

        while j < numbers.len() {
            if numbers[j] < numbers[lowest_number_index] {
                lowest_number_index = j;
            }
            j += 1;
        }

        numbers.swap(i, lowest_number_index);
        i += 1;
    }

    println!("{:?}", numbers);
}
