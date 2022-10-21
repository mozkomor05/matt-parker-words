# Matt Parker's challenge - 5 words with 25 unique letters

A fast solution in rust to Matt Parker's challenge.

## The challenge

The problem is to find 5 words with 25 unique letters. The words must be exactly five letters long. The challenge got
popularized by the viral game Wordle and Matt Parker's video on the topic. I highly suggest to watch the video as well
as the follow-up video about the optimisation.

[![The video](https://img.youtube.com/vi/_-AfhLQfb6w/0.jpg)](https://www.youtube.com/watch?v=_-AfhLQfb6w)
[![The video about optimisation](https://img.youtube.com/vi/c33AZBnRHks/0.jpg)](https://www.youtube.com/watch?v=c33AZBnRHks)

## Optimisation techniques

- Words are represented by 32bit integers. Each bit (in n-th position) represents whether the nth letter is present in
  the word. This allows for fast comparison of words.
- While reading the dictionary each words that doesn't have exactly five letters is skipped.
- The "masks" (integers) are also calculated while reading. The calculation algorithm is:
    - Initialize the mask to 0.
    - For each letter, left shift 1 by the position of the letter in the alphabet.
    - Bitwise AND the mask with the new letter mask. If the result doesn't equal to 0, skip the word.
    - Bitwise OR the mask with the letter mask and save it as the new mask.
- Words are deduplicated and ordered (for faster deduplication and to make use of branch prediction).
- Parallelization is used for the first for-loop.
- The words are filtered using bitwise AND. The result of the filtration is used in the next for-loop.
- Every other for-loop starts from the index of the previous for-loop. This allows for skipping an already checked
  combinations.

## Building and running

To build the project, run:

```bash
cargo build --release
```

You will find the binary in `target/release/five-words-unique-letters`. Alternatively, you can run the project using:

```bash
cargo run --release
```

## Performance

The program is able to find the solution in 700 ms on my machine (for
comparison, [Sylvester's solution](https://github.com/oisyn/parkerwords) takes 200 ms on
my machine).