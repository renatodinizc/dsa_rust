# Rust DS&A Collection

## Overview

This Rust project is a collection of algorithms and data structures, ideal for those looking to explore these concepts in Rust. It includes efficient implementations of Bubble Sort, Selection Sort, Insertion Sort, and Quick Sort algorithms, alongside a custom hash map data structure. The project aims to be a valuable resource for learning and applying data structures and algorithms in Rust.

## Getting Started

Ensure you have Rust installed on your system for running this project. Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/renatodinizc/dsa_rust.git
cd dsa_rust
```

## Usage
This library can be used both as an API for your Rust applications.
To do so, import the modules to utilize the data structures and sorting algorithms you need.

### Examples
```rust
use dsa_rust::insertion_sort;
let sorted_array = insertion_sort::sort(vec![3, 1, 4, 1, 5, 9]);
// Output: [1, 1, 3, 4, 5, 9]
```

```rust
use dsa_rust::quicksort;
let sorted_array = quicksort::sort(vec!["grapes", "potatoes", "apples", "pineapples", "watermelons"]);
// Output: ["apples", "grapes", "pineapples", "potatoes", "watermelons"]
```

```rust
use dsa_rust::data_structures::MyHashMap;

let mut map: MyHashMap<&str, u32> = MyHashMap::new();
map.insert("key1", 108);
map.insert("key2", 2010);
let value = map.get("key1"); // Some(108)
let removed_value = map.remove("key2"); // Some(2010)

```

## Implemented Algorithms

### Bubble Sort

A straightforward algorithm, Bubble Sort repeatedly compares and swaps adjacent elements if they are in the wrong order. Its simplicity makes it suitable for small datasets and educational purposes.

### Selection Sort

This algorithm segments the list into sorted and unsorted parts. It continuously removes the smallest element from the unsorted segment and adds it to the sorted one. It's easy to understand but less efficient for larger lists.

### Insertion Sort

Highly efficient for small or nearly sorted datasets, Insertion Sort builds the final sorted array one item at a time, offering an intuitive approach to sorting.

### Quick Sort

Quick Sort is a highly efficient, divide-and-conquer algorithm, known for its superior performance with large datasets.

## Data Structures Implementation

### Custom HashMap

Our custom hash map implementation demonstrates fundamental hash map operations with efficiency and simplicity, suitable for educational and practical purposes.

## Testing

Test cases are included for verifying the correctness of each algorithm and the hash map. Run tests with:

```bash
cargo test
```

## Contributing

We encourage contributions! Open issues or submit pull requests to enhance existing implementations or add new features.

## License

This project is under the MIT License - see the [LICENSE](LICENSE.md) file for details.
