fn binary_search(arr: &[usize], len: usize, num: usize)-> usize{
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


fn main() {
    println!("Binary Search Operation");

    let arr = [1, 2, 3, 4, 5, 6, 7];

    for (index, &i) in arr.iter().enumerate() {
        let num = i;
        let pos = binary_search(&arr, arr.len(), num); 
    
        if pos == index {
            println!("TEST PASSED : Found {} at pos {}\n", num, pos);
        } else {
            println!("TEST FAILED : Did not found {} at pos {}\n", num, pos);
        }
    }
}
