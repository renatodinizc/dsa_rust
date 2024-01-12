# Rust DS&A Collection

## Overview

This project is a collection of different algorithms and data structures implemented in Rust. Currently, it includes implementations for two sorting algorithms: Bubble Sort and Selection Sort. The project is intended to serve as a learning resource for those interested in algorithms and data structures using the Rust programming language.

## Getting Started

To run the project, make sure you have Rust installed on your machine. You can then clone this repository and navigate to the project directory.

```bash
git clone https://github.com/renatodinizc/dsa_rust.git
cd dsa_rust
```

## Usage

The main application accepts command-line arguments to specify the sorting algorithm and the list of numbers to be sorted. The supported options are:

- `-b`: Bubble Sort
- `-s`: Selection Sort

Example usage:

```bash
cargo run -- -b 123 45 3 28 74 19123 28 28 1
```

## Implemented Algorithms

### Bubble Sort

Bubble Sort is a simple sorting algorithm that repeatedly steps through the list, compares adjacent elements, and swaps them if they are in the wrong order.

```rust
use dsa_rust::bubble_sort;

let numbers = vec![123, 45, 3, 28, 74, 19123, 28, 28, 1];
bubble_sort(numbers);
```

### Selection Sort

Selection Sort is another simple sorting algorithm that divides the input list into a sorted and an unsorted region. It repeatedly selects the smallest (or largest) element from the unsorted region and swaps it with the first element in the unsorted region.

```rust
use dsa_rust::selection_sort;

let numbers = vec![123, 45, 3, 28, 74, 19123, 28, 28, 1];
selection_sort(numbers);
```

## Testing

The project includes test cases to ensure the correctness of the implemented sorting algorithms. You can run the tests using the following command:

```bash
cargo test
```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve the existing implementations or add new algorithms and data structures.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.