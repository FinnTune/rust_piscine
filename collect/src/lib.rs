pub fn bubble_sort(vec: &mut Vec<i32>) {
    let n = vec.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }
}