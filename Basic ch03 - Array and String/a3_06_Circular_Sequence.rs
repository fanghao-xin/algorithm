use std::io::{self,BufRead};
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    for seq in io::stdin().lock().lines().map(|x| x.unwrap()).skip(1) {
        let seq_plus = seq.clone() + &seq[..seq.len()-1];
        println!("{}",(0..seq.len()).map(|x| &seq_plus[x..x+seq.len()]).min().unwrap());
    }
    Ok(())
}
