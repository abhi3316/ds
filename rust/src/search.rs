pub fn binary_search(arr: &[usize], len: usize, num: usize)-> usize{
    let mut start = 0;
    let mut middle;
    let mut end = len - 1;

    while start <= end {
        middle = (start + end) / 2;
        if arr[middle] == num {
            return middle;
        } else if num > arr[middle] {
            start = middle + 1;
        } else {
            end = middle - 1;
        }
    }

    return usize::MAX;
}

