use std::io::{self,BufRead};
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    for line in io::stdin().lock().lines().map(|x| x.unwrap()).skip(1) {
        let max = line.parse()?;
        let mut counter = [0;10];
        (1..=max).map(|x| x.to_string()).for_each(|x| x.chars().for_each(|i| counter[i as usize - '0' as usize]+=1));
        counter.iter().for_each(|x| print!("{} ",x));
        println!();
    }
    Ok(())
}
