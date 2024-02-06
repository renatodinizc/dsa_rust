# Rust DS&A Collection

## Overview

This Rust project is a collection of algorithms and data structures, ideal for those looking to explore these concepts in Rust. It includes efficient implementations of Bubble Sort, Selection Sort, Insertion Sort, Quick Sort and now Merge Sort algorithms, along with custom implementations of a hash map, stack, and queue data structures. The project aims to be a valuable resource for learning and applying data structures and algorithms in Rust.

## Getting Started

Ensure you have Rust installed on your system for running this project. Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/renatodinizc/dsa_rust.git
cd dsa_rust
```

## Usage
This library can be used as an API for your Rust applications.
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
use dsa_rust::hash_map::HashMap;

let mut map: HashMap<&str, u32> = HashMap::new();
map.insert("key1", 108);
map.insert("key2", 2010);
let value = map.get("key1"); // Some(108)
let removed_value = map.remove("key2"); // Some(2010)

```

## Implemented Algorithms

### Bubble Sort

A simple comparison-based algorithm where each pair of adjacent elements is compared and the elements are swapped if they are not in order.
It's suitable for small data sets, but inefficient for larger lists due to its O(n²) average and worst-case complexity.

### Selection Sort

This algorithm sorts an array by repeatedly finding the minimum element from the unsorted part and moving it to the beginning.
Although it has an O(n²) time complexity for all cases, it performs well on small lists.

### Insertion Sort

It builds the final sorted array one item at a time, with efficient performance for small data sets.
This algorithm is adaptive, and its average and worst-case performance is O(n²), making it less efficient for large lists.

### Quick Sort

A highly efficient, divide-and-conquer sorting algorithm. It selects a 'pivot' element and partitions the other elements into two sub-arrays,
according to whether they are less than or greater than the pivot. The sub-arrays are then sorted recursively.
This algorithm is known for its superior performance with an average and worst-case complexity of O(n log n) and O(n²), respectively.

### Merge Sort

Merge Sort is an efficient, stable, comparison-based, divide-and-conquer sorting algorithm. Most implementations produce a stable sort, meaning that the implementation preserves the input order of equal elements in the sorted output. It is particularly good for large data sets where its O(n log n) time complexity outperforms simpler algorithms like Bubble Sort or Insertion Sort.

## Data Structures Implementation

### Custom HashMap

A generic hashmap implementation providing the fundamental operations as insert, get and remove with efficiency and simplicity,

### Custom Stack

A generic stack implementation providing basic stack operations like push, pop, and read, demonstrating a Last In, First Out (LIFO) behavior.

### Custom Queue

A generic queue implementation featuring operations such as enqueue, dequeue, and read, showcasing a First In, First Out (FIFO) behavior.

### Custom LinkedList

A generic and memory-safe linked list implementation, supporting essential operations like peek_at, push_at, and pop_at, with iterator support for efficient element traversal.

### Custom Binary Search Tree
A generic, memory-safe binary search tree custom implementation, supporting essential operations like search, insert and remove.

### Custom Heap
A generic heap implementation featuring basic operations like insert and remove items with O(log N). It is a natural fit for a priority queue implementation.

### Custom Trie
A generic Trie implementation, which can be used to implement a autocomplete feature for words in a smartphone. Pretty cool!

## Testing

Test cases are included for verifying the correctness of each algorithm and the hash map. Run tests with:

```bash
cargo test
```

## Contributing

We encourage contributions! Open issues or submit pull requests to enhance existing implementations or add new features.

## License

This project is under the MIT License - see the [LICENSE](LICENSE.md) file for details.
