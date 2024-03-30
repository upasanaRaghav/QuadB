fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    arr.iter().copied().nth(k - 1)
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    let k = 3;
    let output = kth_smallest(&arr, k);
    println!("{}th smallest element in {:?} is {:?}", k, arr, output);
}
