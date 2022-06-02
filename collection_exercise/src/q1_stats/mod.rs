use std::collections::HashMap;

pub fn compute_stats(integers: &Vec<i32>) -> (f32, f32, i32) {
    let mean = compute_mean(&integers);
    let median = compute_median(&integers);
    let mode = compute_mode(&integers);
    (mean, median, mode)
}

pub fn compute_mean(integers: &Vec<i32>) -> f32 {
    integers.iter().sum::<i32>() as f32 / integers.len() as f32
}

pub fn compute_median(integers: &Vec<i32>) -> f32 {
    let mut integers_tmp = integers.clone();
    integers_tmp.sort();
    if integers_tmp.len() % 2 == 0 {
        let mid = integers_tmp.len() / 2;
        (integers_tmp[mid - 1] + integers_tmp[mid]) as f32 / 2f32
    } else {
        let mid = integers_tmp.len() / 2;
        integers_tmp[mid] as f32
    }
}

pub fn compute_mode(integers: &Vec<i32>) -> i32 {
    let mut count = HashMap::new();
    for &num in integers {
        *count.entry(num).or_insert(0) += 1;
    }

    count
        .into_iter()
        .max_by_key(|&(_, c)| c)
        .map(|(val, _)| val)
        .expect(
            "Cannot
    compute the mode of zero numbers",
        )
}
