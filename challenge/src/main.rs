mod level_up;

use level_up::*; // Import all the re-exported functions from level_up

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}
