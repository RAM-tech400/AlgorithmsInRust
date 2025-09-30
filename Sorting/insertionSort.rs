fn main() {
    let mut list = vec![98, 24, 48, 4, 26, 37, 81, 15, 54, 12, 8, 3, 9, 16];
    println!("Orginal list: {:?}", list);
    insertionSort(&mut list);
    println!("Sorted list: {:?}", list);
}

fn insertionSort(list: &mut Vec<i32>) {
    println!("Insertion sorting...");
    for i in 1..list.len() {
        let key = list[i];
        let mut j = (i - 1) as isize;
        while j >= 0 && key < list[j as usize] {
            list[(j + 1) as usize] = list[j as usize];
            j -= 1;
        }
        list[(j + 1) as usize] = key;
    }
}

