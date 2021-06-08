# sort-rs

A repository with various sorting algorithms

## List of Algorithms

1. Bubble Sort
2. Selection Sort
3. Insertion Sort
4. Merge Sort
4. Quick Sort

## Use as a external crate

```rs
extern crate sort;

use sort::quick_sort;

fn main() {
  let mut list_of_numbers = [4, 2, 7, 5, 1, 3];
  quick_sort(&mut list_of_numbers);
  assert_eq!(list_of_numbers, [1, 2, 4, 5, 7]);
}
```

```rs
extern crate sort;

use sort::insertion_sort;

fn main() {
  let mut list_of_chars = ['s', 'y', 's', 't', 'e', 'm'];
  insertion_sort(&mut list_of_chars)
  assert_eq!(list_of_numbers, ['e', 'm', 's', 's', 't', 'y']);
}
```

## CLI

### Options

```sh
$ cargo run -- <bubble|selection|insertion|merge|quick> <comma-separated list of integers>
```

For example
```sh
$ cargo run -- merge "12,54,33,27,19,23,44"
> [12, 19, 23, 27, 33, 44, 54]
```

### How to build

Make sure you have [rust-toolchain](https://www.rust-lang.org/tools/install) installed to build the executable

For windows only

```sh
$ ./release.sh
```

### How to run

After you have built the executable, run the following
```sh
$ ./build/sorter.exe <algorithm-option> <comma-separated list of integers>
```