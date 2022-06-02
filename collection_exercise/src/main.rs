pub mod q1_stats;
pub mod q2_piglatin;
pub mod q3_employee;

fn main() {
    println!("Testing q1_stats...");
    let numbers: Vec<i32> = vec![9, 1, 2, 3, 3, 5, 1, 1];
    println!("numbers: {:?}", numbers);
    let (mean, median, mode) = q1_stats::compute_stats(&numbers);
    println!("MEAN: {}, MEDIAN: {}, MODE: {}", mean, median, mode);
    println!("Done testing p1_stats.\n");

    println!("Testing q2_piglatin...");
    println!("first -> {}", q2_piglatin::piglatin("first"));
    println!("apple -> {}", q2_piglatin::piglatin("apple"));
    println!("Done testing q2_piglatin.\n");

    println!("Testing q3_employee...");
    q3_employee::simulate_company();
    println!("Done testing q3_employee.");
}
