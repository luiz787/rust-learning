use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    let input = String::from(&input[..input.len() - 1]);

    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    let first_letter = input.chars().next().expect("Empty string.");
    let first_letter_is_vowel = vowels.contains(&first_letter);

    if first_letter_is_vowel {
        let output = input + "-hay";
        println!("{}", output);
    } else {
        let input_without_first = input
            .chars()
            .next()
            .map(|c| &input[c.len_utf8()..])
            .unwrap_or("");

        let output = format!("{}-{}ay", input_without_first, first_letter);
        println!("{}", output);
    }
}
