use dsa_rust::*;

fn main() {
    let args = std::env::args();
    let mut option = String::new();
    let mut numbers: Vec<u32> = Vec::new();

    for arg in args.skip(1) {
        match arg.parse() {
            Ok(number) => numbers.push(number),
            Err(_e) => option = arg,
        }
    }

    if option == "-b" {
        bubble_sort(numbers);
    } else if option == "-s" {
        selection_sort(numbers);
    }
}
