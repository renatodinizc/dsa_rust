pub fn sort(mut numbers: Vec<u32>) {
    let mut lowest_number_index = 0;

    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            if numbers[j] < numbers[lowest_number_index] {
                lowest_number_index = j;
            }
        }
        numbers.swap(i, lowest_number_index);
    }

    println!("{:?}", numbers);
}
