fn main() {
    let mut list = vec![98, 24, 48, 4, 26, 37, 81, 15, 54, 12, 8, 3, 9, 16];
    println!("Orginal list: {:?}", list);
    selectionSort(&mut list);
    println!("Sorted list: {:?}", list);
}

fn selectionSort(list: &mut Vec<i32>) {
    println!("Selection sorting...");
    for i in (0..list.len() - 1) {
        let mut min = i;
        for j in (i + 1.. list.len()) {
            if list[j] < list[min] {
                min = j;
            }
        }
        let tmp = list[i];
        list[i] = list[min];
        list[min] = tmp;
    }
}

