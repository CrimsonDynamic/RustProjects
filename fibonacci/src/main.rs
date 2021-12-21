
fn main() {
    println!("{}",fibonnaci_seq(10));
}

fn fibonnaci_seq(i: i32) -> u64 {
    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _n in 1..i {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    sum
}