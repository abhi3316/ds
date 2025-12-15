mod search;

fn main() {
    println!("Binary Search Operation");

    let arr = [1, 2, 3, 4, 5, 6, 7];

    for (index, &i) in arr.iter().enumerate() {
        let num = i;
        let pos = search::binary_search(&arr, arr.len(), num); 
    
        if pos == index {
            println!("TEST PASSED : Found {} at pos {}\n", num, pos);
        } else {
            println!("TEST FAILED : Did not found {} at pos {}\n", num, pos);
        }
    }
}
