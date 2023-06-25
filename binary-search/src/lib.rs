pub fn find<R: AsRef<[T]>, T: Ord + Copy>(array: R, key: T) -> Option<usize> {
    let array_ref = array.as_ref();

    if array_ref.is_empty() {
        return None;
    }

    let index = recursive_find(&array_ref, key, 0, array_ref.len() - 1);

    if array_ref[index] == key {
        Some(index)
    } else {
        None
    }
}

fn recursive_find<T: Ord>(vector: &[T], key: T, start: usize, end: usize) -> usize {

    if start == end {
        return end;
    }

    let slice_length = end - start + 1;
    let is_even = slice_length % 2 == 0;

    let mid_point = (start + end) / 2;

    let left = mid_point;
    let right = if is_even { mid_point + 1 } else { mid_point };

    if key <= vector[mid_point] {
        recursive_find(&vector, key, start, left)
    } else {
        recursive_find(&vector, key, right, end)
    }
}
