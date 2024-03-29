use std::io::BufRead;

fn print_hashmap<K: std::fmt::Debug, V: std::fmt::Debug>(
    hash_map: &std::collections::HashMap<K, V>,
    msg: String,
) {
    print!(
        "print_hashmap{}: length: {}, hashmap: [ ",
        msg,
        hash_map.len()
    );
    for (key, value) in hash_map {
        print!("{{{:?}: {:?}}} ", key, value);
    }
    print!("]\n");
}

fn fill_freq_items(
    item_counts: &mut std::collections::HashMap<String, i64>,
    lines: &Vec<String>,
    support: i64,
) {
    for line in &lines[1..] {
        let words: Vec<_> = line.split(" ").collect();

        for word in words {
            item_counts
                .entry(word.to_owned())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }
    print_hashmap(&item_counts, "".to_string());

    item_counts.retain(|_, value| *value >= support);
    print_hashmap(&item_counts, " after filter".to_string());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file_name = "input.txt".to_string();
    let input_file = std::fs::File::open(input_file_name)?;
    let lines: Vec<String> = std::io::BufReader::new(input_file)
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    let first_line = lines.get(0).unwrap();
    let first_line_words: Vec<_> = first_line.split(" ").collect();

    let support = first_line_words.get(0).unwrap().parse::<i64>()?;
    println!("main: support: {}", support);

    let confidence = first_line_words.get(1).unwrap().parse::<f64>()?;
    println!("main: confidence: {}", confidence);

    let mut item_counts = std::collections::HashMap::<String, i64>::new();
    fill_freq_items(&mut item_counts, &lines, support);

    let mut pair_counts = std::collections::HashMap::<(String, String), i64>::new();
    for line in &lines[1..] {
        let words: Vec<_> = line.split(" ").collect();

        for i in 0..words.len() {
            for j in i + 1..words.len() {
                let mut left_word = words.get(i).unwrap().to_string();
                let mut right_word = words.get(j).unwrap().to_string();

                if left_word.gt(&right_word) {
                    let temp = right_word.clone();
                    right_word = left_word.clone();
                    left_word = temp.clone();
                }

                if item_counts.contains_key(&left_word) && item_counts.contains_key(&right_word) {
                    pair_counts
                        .entry((left_word.clone(), right_word.clone()))
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                }
            }
        }
    }
    print_hashmap(&pair_counts, "".to_string());

    pair_counts.retain(|_, value| *value >= support);
    print_hashmap(&pair_counts, " after filter".to_string());

    println!("\nAssociation rules:");

    for (key, value) in pair_counts {
        let right_confidence = value as f64 / *(item_counts.get(&key.0).unwrap()) as f64;
        let left_confidence = value as f64 / *(item_counts.get(&key.1).unwrap()) as f64;

        if right_confidence >= confidence {
            println!("{} -> {}, confidence: {:.3}", key.0, key.1, right_confidence);
        }

        if left_confidence >= confidence {
            println!("{} -> {}, confidence: {:.3}", key.1, key.0, left_confidence);
        }
    }

    print!("\n");

    Ok(())
}

