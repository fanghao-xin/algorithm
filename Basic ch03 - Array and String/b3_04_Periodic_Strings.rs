use std::io::{self,BufRead};
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    for line in io::stdin().lock().lines().map(|x| x.unwrap()).skip(1) {
        let mut min_p = 1;
        while min_p<=line.len() {
            if line.len() % min_p == 0 {
                let mut p_str = String::new();
                (1..=line.len()/min_p).for_each(|_| p_str+=&line[..min_p]);
                if p_str==line {println!("{}",min_p);break;}
            }
            min_p += 1;
        }
    }
    Ok(())
}
