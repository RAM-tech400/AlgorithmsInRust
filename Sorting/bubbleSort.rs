fn main() {
    let mut list = vec![98, 24, 48, 4, 26, 37, 81, 15, 54, 12, 8, 3, 9, 16];
    println!("Orginal list: {:?}", list);
    bubbleSort(&mut list);
    println!("Sorted list: {:?}", list);
}

// A simple bubble sort for i32 lists.
fn bubbleSort(list: &mut Vec<i32>) {
    println!("Bubble sorting...");
    for i in (1..list.len()).rev() {
        for j in (0..i) {
            if list[j] > list[j+1] {
                let tmp = list[j];
                list[j] = list[j+1];
                list[j+1] = tmp;
            }
        }
    }
}

