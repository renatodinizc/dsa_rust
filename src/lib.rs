pub fn bubble_sort(mut numbers: Vec<u32>) {
    let mut done: bool = false;

    while !done {
        let mut i = 0;
        let mut j = 1;
        done = true;

        while j < numbers.len() {
            if numbers[i] > numbers[j] {
                numbers.swap(i, j);
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

        numbers.swap(i, lowest_number_index);
        i += 1;
    }

    println!("{:?}", numbers);
}

pub fn insertion_sort(mut numbers: Vec<u32>) {
    for i in 1..numbers.len() {
        let temp = numbers[i];

        for j in (0..i).rev() {
            if numbers[j] > temp {
                numbers.swap(j, j + 1);
            } else {
                break;
            }
        }
    }

    println!("{:?}", numbers);
}
