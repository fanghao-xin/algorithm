use std::io::{self,BufRead};
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    let mut max = 0;
    let mut nums = vec![0;0];
    for num in io::stdin().lock().lines().map(|x| x.unwrap().parse().unwrap()).skip(1) {
        if num>max {max = num;}
        nums.push(num);
    }
    let mut digit_generator = vec![0;max+1];
    for i in 1..=max {
        let mut p = i; let mut d = i;
        while p>0 { d += p % 10; p = p / 10; }
        if d<=max && digit_generator[d]==0 {digit_generator[d] = i;}
    }
    nums.iter().for_each(|&x| println!("{}",digit_generator[x]));
    Ok(())
}