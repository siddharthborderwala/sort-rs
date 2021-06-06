extern crate sort;

use sort::*;
use std::io::Write;
use std::time::Instant;

fn main() {
  let name = match std::env::args().nth(1) {
    Some(v) => v,
    None => {
      std::io::stderr()
        .write(b"Please pass an algorithm name <bubble|insertion|selection|merge|quick>")
        .unwrap();
      return;
    }
  };
  let algorithm = name.as_str();
  let string_list = match std::env::args().nth(2) {
    Some(v) => v,
    None => {
      std::io::stderr()
        .write(b"Please pass an array of integers (e.g. \"4,2,7,11,9\")")
        .unwrap();
      return;
    }
  };
  let mut list = string_list
    .split(",")
    .map(|i| i.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();
  match algorithm {
    "bubble" => measure(&mut || bubble_sort(&mut list)),
    "insertion" => measure(&mut || insertion_sort(&mut list)),
    "selection" => measure(&mut || selection_sort(&mut list)),
    "merge" => measure(&mut || merge_sort(&mut list)),
    "quick" => measure(&mut || quick_sort(&mut list)),
    _ => {
      std::io::stderr()
        .write(b"Please pass a valid algorithm name <bubble|insertion|selection|merge|quick>")
        .unwrap();
      return;
    }
  }
  println!("Sorted list:  {:?}", list);
}

fn measure(func: &mut dyn FnMut() -> ()) {
  let start = Instant::now();
  func();
  let elapsed_nano = start.elapsed().as_nanos();
  println!("Time elapsed: {} nanoseconds", elapsed_nano);
}
