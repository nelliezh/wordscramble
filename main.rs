use std::sync::Mutex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use itertools::Itertools;
use num_cpus;

fn main() -> io::Result<()> {
    // Open the dictionary
    let cpus = num_cpus::get();
    let file = File::open("words.txt")?;
    let reader = BufReader::new(file);

    // Create a map from word length to a set of words with that length.
    let mut words:HashMap<usize, HashSet<String>> = HashMap::new();

    // Read all words into the map.
    for line in reader.lines() {
        let l = line?.to_lowercase();
        if let Some(words_with_len) = words.get_mut(&l.len()) {
            words_with_len.insert(l);
        } else {
            let mut v = HashSet::new();
            let len = l.len();
            v.insert(l);
            words.insert(len, v);
        };
    }

    // Iterate over arguments, skipping the first, which is the program name.
    for arg in std::env::args().skip(1) {
        // Get the words of the right length
        let words_of_right_length = &words[&arg.len()];

        // Create a map to track what words have already been found.
        let seen_words = Mutex::new(HashSet::new());

        // Create a set of scoped threads to parallelize the searching work.
        std::thread::scope(|s| {
            let arg = &arg;
            let seen_words = &seen_words;
            // Create one thread for each cpu.
            for thread_index in 0..cpus {
                s.spawn(move || {
                    // Iterate over all permutations and see if the permutation is present in the dictionary.
                    // For thread X, we skip X elements at the beginning, and from there on, we
                    // skip by the number of cpus, so if there are 3 cpus:
                    // thread 0 takes 0, 3, 6, 9, ...
                    // thread 1 takes 1, 4, 7, 10, ...
                    // thread 2 takes 2, 5, 8, 11, ...
                    for p in arg.chars().permutations(arg.len()).skip(thread_index).step_by(cpus) {
                        // Collect the letters back into a word.
                        let perm_word = p.iter().collect::<String>();
                        if words_of_right_length.contains(&perm_word) {
                            // Found a word that matches. See if it was already seen.
                            {
                                let mut locked_map = seen_words.lock().unwrap();
                                // It was, just skip it.
                                if locked_map.contains(&p) {
                                    continue;
                                }
                                // If we got here, it's a new word. Add it to the map, then print
                                // it out.
                                locked_map.insert(p);
                            }
                            println!("{}", &perm_word);
                        }
                    }
                });
            }
        });
    }

    Ok(())
}
