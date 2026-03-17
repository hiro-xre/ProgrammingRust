fn partition<T: Ord>(slice: &mut [T]) -> usize {
    let len = slice.len();
    let pivot_index = len - 1;
    let mut i = 0;

    for j in 0..pivot_index {
        if slice[j] <= slice[pivot_index] {
            slice.swap(i, j);
            i += 1;
        }
    }

    slice.swap(i, pivot_index);
    i
}

fn quicksort<T: Ord>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }

    let pivot_index = partition(slice);

    quicksort(&mut slice[..pivot_index]);

    quicksort(&mut slice[pivot_index + 1..]);
}

fn main() {
    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    quicksort(&mut v);
    println!("{:?}", v);
}
