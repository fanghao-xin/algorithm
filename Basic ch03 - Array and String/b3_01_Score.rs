use std::io::{self,BufRead};
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    for line in io::stdin().lock().lines().map(|x| x.unwrap()).skip(1) {
        let score = line.chars().fold((0,0),|mut s,x| {
            s.0 += if x=='O' {s.1+=1;s.1} else {s.1=0;0};
            (s.0,s.1)
        }).0;
        println!("{}",score);
    }
    Ok(())
}
