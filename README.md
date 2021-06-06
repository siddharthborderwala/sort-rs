# sort-rs

A repository with various sorting algorithms

## List of Algorithms

1. Bubble Sort
2. Selection Sort
3. Insertion Sort
4. Merge Sort
4. Quick Sort

## CLI Options

```sh
$ cargo run -- <bubble|selection|insertion|merge|quick> <comma-separated list of integers>
```

For example
```sh
$ cargo run -- merge "12,54,33,27,19,23,44"
> [12, 19, 23, 27, 33, 44, 54]
```

## How to build

Make sure you have [rust-toolchain](https://www.rust-lang.org/tools/install) installed to build the executable

```sh
$ ./release.sh
```

## How to run

After you have built the executable, run the following
```sh
$ ./build/sorter.exe <algorithm-option> <comma-separated list of integers>
```