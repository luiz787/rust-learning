use std::collections::HashMap;

fn main() {
    let integers: Vec<u32> = vec![1, 2, 3, 5, 6, 87, 43, 32, 65, 65];
    let mut sum = 0;
    for integer in &integers {
        sum = sum + integer;
    }
    println!(
        "Mean: {}\nMedian: {}\nMode: {}",
        mean(&integers),
        median(&integers),
        mode(&integers)
    );
}

fn mean(numbers: &Vec<u32>) -> f32 {
    numbers.iter().sum::<u32>() as f32 / numbers.len() as f32
}

fn median(numbers: &Vec<u32>) -> u32 {
    let mut sorted_integers = numbers.clone();
    sorted_integers.sort();

    let median_index: usize = sorted_integers.len() / 2;
    sorted_integers[median_index]
}

fn mode(integers: &Vec<u32>) -> u32 {
    let mut occurrences: HashMap<u32, u32> = HashMap::new();
    for &val in integers {
        *occurrences.entry(val).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}
