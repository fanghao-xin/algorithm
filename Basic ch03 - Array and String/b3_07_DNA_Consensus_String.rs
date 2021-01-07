use std::io::{self,BufRead};
use std::error::Error;
use std::collections::BTreeMap;

fn main() -> Result<(),Box<dyn Error>> {
    let (mut m, mut n) = (0,0);
    let mut hamming:Vec<BTreeMap<char,i32>> = Vec::new();
    for line in io::stdin().lock().lines().map(|x| x.unwrap()).skip(1) {
        if m==0 {
            let line = line.split_whitespace().collect::<Vec<_>>();
            m = line[0].parse()?;
            n = line[1].parse()?;
            hamming = vec!["ACGT".chars().zip(vec![0;4].into_iter()).collect();n];
            n = m;
        } else {
            line.char_indices().for_each(|(i,x)| *hamming[i].entry(x).or_insert(0) += 1 );
            m -= 1;
            if m==0 {
                let mut sum = 0;
                for h in &hamming {
                    let mut max_count = -1;
                    let mut max_char = ' ';
                    for (&k,&v) in h {
                        if v > max_count {
                            max_count = v;
                            max_char = k;
                        }
                    }
                    print!("{}",max_char);
                    sum += n as i32 - max_count;
                }
                println!("\n{}",sum);
            }
        }
    }
    Ok(())
}
