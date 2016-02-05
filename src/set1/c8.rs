use util::ioutil;

// SET 1, CHALLENGE 8: http://cryptopals.com/sets/1/challenges/8/

pub fn main() {
    let lines = ioutil::read_lines_hex("res/1-8.txt").unwrap();
    let (line_num, score) = lines.iter()
        .enumerate()
        .map(|(i, line)| (i, count_self_similarity(line)))
        //.inspect(|&(i, score)| println!("Line: {}; score: {}", i, score))
        .max_by_key(|&(_i, count)| count).unwrap();

    println!("Best candidate is line {} with self-similarity score of {}.", line_num, score);
}

fn count_self_similarity(data: &Vec<u8>) -> usize {
    let mut count = 0;
    let mut blocks: Vec<&[u8]> = Vec::new();
    for block in data.chunks(16) {
        for block2 in &blocks {
            if &&block == &block2 { // &&refs pls
                count += 1;
            }
        }
        blocks.push(block);
    }
    count
}
