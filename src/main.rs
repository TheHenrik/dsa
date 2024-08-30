mod binary_search;
mod liniar_search;

fn main() {
    println!("Hello, world!");
    let a = vec![1, 2, 3, 4, 5];
    println!(
        "Binary Search found: {:?}",
        binary_search::bin_search(&a, 1)
    );
    println!(
        "Linear Search found: {:?}",
        liniar_search::lin_search(&a, 3)
    );
    println!(
        "Count of value: {:?}",
        liniar_search::count(&a, 3)
    );
}
