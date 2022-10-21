use rayon::prelude::*;
use std::{collections, env, fs};
use std::io::{BufRead, BufReader, Write};
use std::time::Instant;

struct Solver {
    // Binary mask where each 1 bit represents a position of a letter to word
    mask_to_word: collections::HashMap<u32, String>,

    // List of all valid words as binary masks
    valid_masks: Vec<u32>,

    solutions: Vec<[u32; 5]>,
}

impl Default for Solver {
    fn default() -> Self {
        Self {
            mask_to_word: collections::HashMap::new(),
            valid_masks: Vec::new(),
            solutions: Vec::new(),
        }
    }
}

trait WordFinder {
    fn process_file(&mut self, reader: &mut BufReader<fs::File>);

    fn filter_vec<'a>(vec: &'a mut Vec<u32>, prev_vec: &Vec<u32>, mask: u32, start: usize) -> &'a Vec<u32>;

    fn find_solutions(&mut self);

    fn get_solutions(&self) -> Vec<[String; 5]>;
}

impl WordFinder for Solver {
    fn process_file(&mut self, reader: &mut BufReader<fs::File>) {
        for line in reader.lines() {
            let word = line.unwrap().trim().to_owned();

            if word.len() != 5 {
                continue;
            }

            let mut unique = true;
            let mut mask = 0u32;

            for c in word.chars() {
                let bit = 1 << (c as u32 - 'a' as u32); // add 1 to the bit corresponding to the character
                if mask & bit > 0 { // bit is already set => letter appeared twice
                    unique = false;
                    break;
                }
                mask |= bit;
            }

            if unique {
                self.mask_to_word.insert(mask, word);
                self.valid_masks.push(mask);
            }
        }

        // sorting makes it possible to use dedup and also potentially improves performance (branch prediction)
        self.valid_masks.sort();
        self.valid_masks.dedup();
    }

    fn filter_vec<'a>(vec: &'a mut Vec<u32>, prev_vec: &Vec<u32>, mask: u32, start: usize) -> &'a Vec<u32> {
        vec.clear();
        vec.extend(
            prev_vec.iter()
                .skip(start) // skip already checked elements (to increase performance and avoid permutations)
                .filter(|&m| m & mask == 0) // any non-zero values indicates that at least two bits are 1 in the same position => at least one letter is shared
        );
        vec
    }

    fn find_solutions(&mut self) {
        self.solutions = self.valid_masks.par_iter().enumerate().flat_map(|(i, &mask)| { // parallel iteration
            let mut solutions = Vec::new();

            // pre-allocation of vectors to avoid allocations in the loop
            // it is not convenient to allocate vectors outside the parallel loop because then we would have to use a mutex to access them
            let words1 = &mut Vec::with_capacity(self.valid_masks.len());
            let words2 = &mut Vec::with_capacity(self.valid_masks.len());
            let words3 = &mut Vec::with_capacity(self.valid_masks.len());
            let words4 = &mut Vec::with_capacity(self.valid_masks.len());

            // nested iteration is around ~80% faster than recursion
            Self::filter_vec(words1, &self.valid_masks, mask, i);
            for (j, &mask1) in words1.iter().enumerate() {
                Self::filter_vec(words2, words1, mask1, j);
                for (k, &mask2) in words2.iter().enumerate() {
                    Self::filter_vec(words3, words2, mask2, k);
                    for (l, &mask3) in words3.iter().enumerate() {
                        Self::filter_vec(words4, words3, mask3, l);
                        for &mask4 in words4.iter() {
                            solutions.push([mask, mask1, mask2, mask3, mask4]);
                        }
                    }
                }
            }

            solutions
        }).collect();
    }

    fn get_solutions(&self) -> Vec<[String; 5]> {
        self.solutions.iter().map(|solution| {
            let mut arr: [String; 5] = Default::default();
            for (i, &mask) in solution.iter().enumerate() {
                arr[i] = self.mask_to_word.get(&mask).unwrap().to_owned();
            }
            arr
        }).collect()
    }
}

fn main() {
    let timer_total = Instant::now();

    let mut solver = Solver::default();

    let file = fs::File::open("words_alpha.txt").expect("File not found");
    let mut reader = BufReader::new(file);

    solver.process_file(&mut reader);

    let elapsed_preprocessing = timer_total.elapsed();
    let timer_solving = Instant::now();

    solver.find_solutions();

    let solutions = solver.get_solutions();

    let filename = env::args().nth(1).unwrap_or("output.txt".to_owned());
    let mut file = fs::File::create(filename).expect("Unable to create file");
    for solution in &solutions {
        writeln!(file, "{}", solution.join(",")).expect("Unable to write data");
    }

    println!("Found {} solutions.", solutions.len());
    println!("Solutions written to output.txt");
    println!("Preprocessing took: {:?}", elapsed_preprocessing);
    println!("Solving took: {:?}", timer_solving.elapsed());
    println!("Total time: {:?}", timer_total.elapsed());
}