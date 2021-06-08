pub fn bubble_sort<T: PartialOrd>(list: &mut [T]) {
    let size = list.len();
    for i in 0..(size - 1) {
        let mut swapped = false;
        for j in 0..(size - 1 - i) {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

pub fn selection_sort<T: PartialOrd>(list: &mut [T]) {
    let size = list.len();
    for i in 0..(size - 1) {
        let mut min_index = i;
        for j in (i + 1)..(size) {
            if list[j] < list[min_index] {
                min_index = j;
            }
        }
        list.swap(min_index, i);
    }
}

pub fn insertion_sort<T: PartialOrd + Copy>(list: &mut [T]) {
    for i in 1..(list.len()) {
        let key = list[i];
        let mut j = (i - 1) as i32;
        while j >= 0 && list[j as usize] > key {
            list[(j + 1) as usize] = list[j as usize];
            j -= 1;
        }
        list[(j + 1) as usize] = key;
    }
}

fn merge<T: PartialOrd + Copy>(first_half: &[T], second_half: &[T], result: &mut Vec<T>) {
    let s1 = first_half.len();
    let s2 = second_half.len();
    let mut i1 = 0_usize;
    let mut i2 = 0_usize;
    loop {
        if i1 >= s1 {
            while i2 < s2 {
                result.push(second_half[i2]);
                i2 += 1;
            }
            break;
        } else if i2 >= s2 {
            while i1 < s1 {
                result.push(first_half[i1]);
                i1 += 1;
            }
            break;
        }
        if first_half[i1] <= second_half[i2] {
            result.push(first_half[i1]);
            i1 += 1;
        } else {
            result.push(second_half[i2]);
            i2 += 1;
        }
    }
}

fn recursive_merge_sort<T: PartialOrd + Copy>(list: &[T], result: &mut Vec<T>) {
    let size = list.len();
    if size == 1 {
        result.push(list[0]);
        return;
    }
    let middle = size / 2;
    let mut first_half = Vec::with_capacity(middle);
    recursive_merge_sort(&list[..middle], &mut first_half);
    let mut second_half = Vec::with_capacity(size - middle);
    recursive_merge_sort(&list[middle..], &mut second_half);
    let mut acc: Vec<T> = Vec::with_capacity(size);
    merge(&first_half, &second_half, &mut acc);
    for i in acc {
        result.push(i);
    }
}

pub fn merge_sort<T: PartialOrd + Copy>(list: &mut [T]) {
    let mut result: Vec<T> = Vec::with_capacity(list.len());
    recursive_merge_sort(list, &mut result);
    list.copy_from_slice(&result);
}

fn partition<T: PartialOrd + Copy>(list: &mut [T], left_most: i32, right_most: i32) -> i32 {
    let pivot = list[right_most as usize];
    let mut i = left_most - 1;
    let rm_usize = right_most as usize;
    let lm_usize = left_most as usize;

    for j in lm_usize..(rm_usize + 1) {
        if list[j] < pivot {
            i += 1;
            list.swap(i as usize, j);
        }
    }
    list.swap((i + 1) as usize, rm_usize);
    return i + 1;
}

fn recursive_quick_sort<T: PartialOrd + Copy>(list: &mut [T], left_most: i32, right_most: i32) {
    if left_most < right_most {
        let p_index = partition(list, left_most, right_most);
        recursive_quick_sort(list, left_most, p_index - 1);
        recursive_quick_sort(list, p_index + 1, right_most);
    }
}

pub fn quick_sort<T: PartialOrd + Copy>(list: &mut [T]) {
    recursive_quick_sort(list, 0, list.len() as i32 - 1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort() {
        let mut v = [3, 2, 5, 6, 1, 4];
        bubble_sort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_selection_sort() {
        let mut v = [4, 6, 3, 1, 5, 2];
        selection_sort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut v = [1, 5, 3, 6, 2, 4];
        insertion_sort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_sort() {
        let mut v = [2, 3, 4, 1, 6, 5];
        merge_sort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_quick_sort() {
        let mut v = [6, 1, 2, 5, 4, 3];
        quick_sort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5, 6]);
    }
}
