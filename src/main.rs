
pub fn insertion_sort(values: &mut [i32]) {
    // For all values.
    for i in 0..values.len() {
        // Insert the current value into the start of the array.
        for j in (0..i).rev() {
            if values[j] < values[j+1] { break }
            values.swap(j, j+1);
        }
    }
}

pub fn bubble_sort(values: &mut [i32]) {
    for i in 0..values.len() {
        for j in i+1..values.len() {
            if values[i] > values[j] { values.swap(i, j) }
        }
    }
}

pub fn merge_sort(values: &mut [i32], tmp: &mut [i32]) {
    let len = values.len();
    if len < 4 {
        // Below a certain size, do an insertion sort.
        insertion_sort(values);
    } else {
        // Above a certain size, split into two and sort separately.
        let mid = len / 2;
        merge_sort(&mut values[0..mid], &mut tmp[0..mid]);
        merge_sort(&mut values[mid..len], &mut tmp[mid..len]);

        // Duplicate the values and merge them.
        tmp.copy_from_slice(values);
        values
            .iter_mut()
            .zip(itertools::merge(&tmp[0..mid], &tmp[mid..len]))
            .for_each(|(d, s)| *d = *s);
    }
}

pub fn median(mut x: i32, mut y: i32, mut z: i32) -> i32 {
    if x > y { std::mem::swap(&mut x, &mut y); }
    if y > z { std::mem::swap(&mut y, &mut z); }
    if x > y { std::mem::swap(&mut x, &mut y); }
    y
}

pub fn bubble_sort_3(values: [i32; 3]) -> [i32; 3] {
    let [mut x, mut y, mut z] = values;
    if x > y { std::mem::swap(&mut x, &mut y); }
    if y > z { std::mem::swap(&mut y, &mut z); }
    if x > y { std::mem::swap(&mut x, &mut y); }
    [x, y, z]
}

pub fn quick_sort(values: &mut [i32]) {
    let len = values.len();
    if len < 4 {
        // Below a certain size, do a bubble sort.
        bubble_sort(values);
    } else {
        // Guess a pivot value to partition the array.
        let pivot = median(values[0], values[len / 2], values[len-1]);

        // Partition into 3 parts [0..p] [p..q] and [q..len]
        let p = itertools::partition(values.iter_mut(), |&i| i < pivot);
        let q = itertools::partition(values[p..].iter_mut(), |&i| i == pivot) + p;

        // Sort the two unsorted partitions (no need to sort values[p..q]).
        quick_sort(&mut values[0..p]);
        quick_sort(&mut values[q..len]);
    }
}

pub fn rand_vector(size: usize) -> Vec<i32> {
    use rand::distributions::{Distribution, Uniform};
    let between = Uniform::from(1..=10);
    let mut rng = rand::thread_rng();
    (0..size).map(|_| between.sample(&mut rng)).collect()
}

fn main() {
    let mut values = rand_vector(30);
    bubble_sort(&mut *values);
    println!("{:?}", values);
    
    let mut values = rand_vector(30);
    insertion_sort(&mut *values);
    println!("{:?}", values);
    
    let mut values = rand_vector(30);
    let mut tmp = vec![0; values.len()];
    merge_sort(&mut *values, &mut *tmp);
    println!("{:?}", values);

    let mut values = rand_vector(30);
    quick_sort(&mut *values);
    println!("{:?}", values);
}

