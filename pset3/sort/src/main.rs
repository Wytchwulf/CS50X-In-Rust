fn main() {
    let mut numbers = vec![64, 34, 25, 17, 12, 22, 1, 100];
    sort(&mut numbers);
    println!("{:?}", numbers);
}

fn sort(arr: &mut Vec<i32>) {
    for i in 0..arr.len() {
        for j in 0..(arr.len() - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
