fn check_subarray(arr: &[i32], subarr: &[i32]) -> bool {
    let mut i = 0;
    let mut j = 0;
    let arr_len = arr.len();
    let sub_len = subarr.len();

    while i < arr_len /*and j < subarr.len()*/ {
        if arr[i] == subarr[j] {
            i += 1;
            j += 1;
            if j == sub_len {
                return true
            }
        } else {
            i += 1;
            j = 0;
        }
    }

    false
}


fn main() {
    let arr: [i32; 10] = [1, 2, 3, 5, 6, 8, 10, 11, 9, 2];
    let subarr: [i32; 3] = [2,3,6];
    println!("{}", check_subarray(&arr, &subarr));
}
