pub fn bubble_sort(mut numbers: Vec<u32>) {
    let mut done: bool = false;

    while !done {
        let mut i = 0;
        let mut j = 1;
        done = true;

        while j < numbers.len() {
            let (a, b) = (numbers[i], numbers[j]);

            if a > b {
                numbers[j] = a;
                numbers[i] = b;
                done = false;
            }

            i += 1;
            j += 1;
        }
    }
    println!("{:?}", numbers);
}

pub fn selection_sort(mut numbers: Vec<u32>) {
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

        let (a, b) = (numbers[i], numbers[lowest_number_index]);
        numbers[i] = b;
        numbers[lowest_number_index] = a;
        i += 1;
    }

    println!("{:?}", numbers);
}
