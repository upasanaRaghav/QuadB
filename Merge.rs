fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(arr1.len() + arr2.len());
    let mut i = 0;
    let mut j = 0;
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }
    result.extend_from_slice(&arr1[i..]);
    result.extend_from_slice(&arr2[j..]);
    result
}

fn main() {
    let arr1 = vec![1, 3, 5, 7];
    let arr2 = vec![2, 4, 6, 8];
    let merged_array = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged Array: {:?}", merged_array);
}
