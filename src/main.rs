mod binary_search;

fn main() {
    println!("Hello, world!");
    let a = vec![1, 2, 3, 4, 5];
    println!(
        "Binary Search found: {:?}",
        binary_search::bin_search(&a, 1)
    )
}
