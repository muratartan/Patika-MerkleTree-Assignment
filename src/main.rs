#![allow(unused_assignments, non_snake_case)]

use std::fs::read_to_string;

use sha2::{Digest, Sha256};

fn merkle_root(filename: String) -> String {
    let input_data = read_to_string(filename).expect("error when reading the file");

    let mut vector_of_strings: Vec<String> = input_data
        .lines()
        .into_iter()
        .map(|line| line.to_string())
        .collect();

    let n = vector_of_strings.remove(0).parse::<u32>().unwrap();

    let mut hexed_vector: Vec<String> = Vec::new();

    for _ in 0..n {
        hexed_vector = vector_of_strings
            .iter()
            .map(|i: &String| hash_input(i))
            .collect();
        vector_of_strings = create_next_level(hexed_vector);
    }

    let root = hash_input(&vector_of_strings[0]);
    root
}

fn create_next_level(current_level: Vec<String>) -> Vec<String> {
    let mut combined_string = String::new();
    let mut node_vec: Vec<String> = Vec::new();

    for (i, v) in current_level.iter().enumerate() {
        if i % 2 == 0 {
            combined_string = v.clone();
        } else {
            combined_string.push_str(&v);
            node_vec.push(combined_string.clone());
        }
    }

    node_vec
}

fn hash_input(a: &str) -> String {
    let mut hasher = Sha256::new();
    let input = a;
    hasher.update(input);
    let hash = hasher.finalize();
    let hex = hex::encode(&hash);

    return hex.to_string();
}

fn main() {
    let input0_root = merkle_root("input0.txt".to_string());
    println!("The root hash of input0 file is: {}", input0_root);

    let input1_root = merkle_root("input1.txt".to_string());
    println!("The root hash of input1 file is: {}", input1_root);

    let input2_root = merkle_root("input2.txt".to_string());
    println!("The root hash of input2 file is: {}", input2_root);

    let input3_root = merkle_root("input3.txt".to_string());
    println!("The root hash of input3 file is: {}", input3_root);

    let input4_root = merkle_root("input4.txt".to_string());
    println!("The root hash of input4 file is: {}", input4_root);
}

// Pass all tests!
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input_0() {
        let result = merkle_root("input0.txt".to_string());
        assert_eq!(
            result,
            "ff41418be11ed77612aeb83ee0bcf97a5853a4c291e23bd4d4cc6435fcfabdf9"
        );
    }

    #[test]
    fn input_1() {
        let result = merkle_root("input1.txt".to_string());
        assert_eq!(
            result,
            "98a77b2c3ff5f6c2aca697f60b2aa2a1a2733be36dbd35bae23d517c2ad5985e"
        );
    }

    #[test]
    fn input_2() {
        let result = merkle_root("input2.txt".to_string());
        assert_eq!(
            result,
            "3c0fb0638de91551eae4e9d984d72034aa0693be37b51737e6b81bc489866e5e"
        );
    }

    #[test]
    fn input_3() {
        let result = merkle_root("input3.txt".to_string());
        assert_eq!(
            result,
            "f03b1c9163babeb728ac011fe0c2c9c69700a2f8ddde211ec07d621cdb322cfe"
        );
    }

    #[test]
    fn input_4() {
        let result = merkle_root("input4.txt".to_string());
        assert_eq!(
            result,
            "f83e74742fda659dfc07615881af796abafc434f591aeb23b9f4366abe03c597"
        );
    }
}
