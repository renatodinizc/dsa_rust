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
