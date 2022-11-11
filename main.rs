use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::env;
use itertools::Itertools;

fn main() -> io::Result<()> {
    // Open the dictionary
    let file = File::open("/usr/share/dict/words")?;
    let reader = BufReader::new(file);
    let mut words:HashMap<usize, HashSet<String>> = HashMap::new();

    // Read all words into a map<word_length, set<words>>
    for line in reader.lines() {
        let l = line?.to_lowercase();
        if let Some(words_with_len) = words.get_mut(&l.len()) {
            words_with_len.insert(l.clone());
        } else {
            let mut v = HashSet::new();
            v.insert(l.clone());
            words.insert(l.len(), v);
        };
    }

    // Iterate over arguments
    for arg in env::args().skip(1) {
        // Get the words of the right length
        let words_of_right_length = &words[&arg.len()];

        // Set up a set for detecting duplicates.
        let mut seen_words = HashSet::new();

        // Iterate over all permutations and see if the permutation is present in the dictionary
        for p in arg.chars().permutations(arg.len()) {
            if seen_words.contains(&p) {
                continue;
            }
            let perm_word = p.iter().collect::<String>();
            if words_of_right_length.contains(&perm_word) {
                println!("{}", &perm_word);
            }
            seen_words.insert(p);
        }
    }

    Ok(())
}
