use std::io::{self, BufRead};

/*
 * Complete the 'minimumBribes' function below.
 *
 * The function accepts INTEGER_ARRAY q as parameter.
 */

fn minimumBribes(q: &[i32]) {
    let mut sum = 0;
    let mut arr = Vec::from_iter(1..=q.len() as i32);
    for x in q {
        let index_x = arr.binary_search(x).unwrap();
        if index_x > 2 {
            println!("Too chaotic");
            return;
        }
        sum += index_x;
        arr.remove(index_x);
    }
    println!("{}", sum);
}

fn main() {
    minimumBribes(&[4, 1, 2, 3]);
}
