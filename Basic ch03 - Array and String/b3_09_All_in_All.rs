use std::io::{self,BufRead};
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    'outer_loop:
    for line in io::stdin().lock().lines().map(|x| x.unwrap()) {
        let line = line.split_whitespace().collect::<Vec<_>>();
        let a = line[0];
        let mut b = line[1];
        for c in a.chars() {
            if let Some(i) = b.find(c) {
                b = &b[i+1..];
            } else {
                println!("No");
                continue 'outer_loop;
            }
        }
        println!("Yes");
    }
    Ok(())
}
