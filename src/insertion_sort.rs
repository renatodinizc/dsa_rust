pub fn sort(mut numbers: Vec<u32>) {
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
