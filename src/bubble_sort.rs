pub fn sort(mut numbers: Vec<u32>) {
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
