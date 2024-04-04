fn sample_probability_distribution(probabilities: &[f64]) -> usize {
    let random_number = rand::random::<f64>();
    let mut sum = 0.0;
    for (i, &probability) in probabilities.iter().enumerate() {
        sum += probability;
        if random_number < sum {
            return i;
        }
    }
    panic!("The probabilities sum to less than 1.0");
}

fn vec_to_probabilities(vec: &[f64]) -> Vec<f64> {
    let sum: f64 = vec.iter().sum();
    vec.iter().map(|x| x / sum).collect()
}

fn main() {
    println!("Hello, world!");
    let random_number = rand::random::<u8>();
    println!("Random number: {}", random_number);

    // random number between 0 and 1
    let random_number = rand::random::<f64>();
    println!("Random number: {}", random_number);

    //generate vec of random numbers
    let mut random_numbers: Vec<f64> = Vec::new();
    for _ in 0..5 {
        let random_number = rand::random::<f64>();
        random_numbers.push(random_number);
    }

    println!("Random numbers: \t\t{:?}", random_numbers);

    // normalize into a probability distribution
    let sum: f64 = random_numbers.iter().sum();
    let normalized_random_numbers: Vec<f64> = random_numbers.iter().map(|x| x / sum).collect();

    println!(
        "Normalized random numbers: \t{:?}",
        normalized_random_numbers
    );

    // sample from the probability distribution
    let mut counts = vec![0.0; normalized_random_numbers.len()];
    for _ in 0..100000000 {
        let sample = sample_probability_distribution(&normalized_random_numbers);
        counts[sample] += 1.0;
    }

    let probabilities = vec_to_probabilities(&counts);
    println!("Counts: \t\t\t{:?}", counts);
    println!("Probabilities: \t\t\t{:?}", probabilities);
}
