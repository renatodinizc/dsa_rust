use dsa_rust::*;
use dsa_rust::dictionary::Dictionary;

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
        bubble_sort::sort(numbers);
    } else if option == "-s" {
        selection_sort::sort(numbers);
    } else if option == "-i" {
        insertion_sort::sort(numbers);
    }


    let mut dict: Dictionary<&str, u32> = Dictionary::new();

    dict.insert("banana", 10);
    dict.insert("uva", 123);
    dict.insert("laranja", 3);

    println!("laranja: {:?}", dict.get("laranja"));
    println!("banana: {:?}", dict.get("banana"));
    println!("uva: {:?}", dict.get("uva"));
    println!("sabonete: {:?}", dict.get("sabonete"));
    println!("batata doce: {:?}", dict.get("batata doce"));

    println!("valor deletado: {:?}", dict.delete("laranja"));
    println!("apos deletar laranja: {:?}", dict.get("laranja"));



}
