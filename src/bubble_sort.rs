pub fn sort(mut numbers: Vec<u32>) {
    let mut done: bool = false;

    while !done {
        done = true;
        for (i, j) in (1..numbers.len()).enumerate() {
            if numbers[i] > numbers[j] {
                numbers.swap(i, j);
                done = false;
            }
        }
    }

    println!("{:?}", numbers);
}
