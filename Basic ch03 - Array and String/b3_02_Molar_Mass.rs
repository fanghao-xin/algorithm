use std::io::{self,BufRead};
use std::error::Error;
use std::collections::BTreeMap;

fn main() -> Result<(),Box<dyn Error>> {
    let molar_mass:BTreeMap<_,_> ="CHON".chars().zip(&[12.01,1.008,16.00,14.01]).collect();
    for line in io::stdin().lock().lines().map(|x| x.unwrap()).skip(1) {
        let mut i = 0; let mut d = 1; let mut sum = 0.0;
        for c in line.chars().rev() {
            match c {
                'C'|'H'|'O'|'N' => {sum+=molar_mass[&c]*(if i!=0 {i as f64} else{1.0}); i=0; d=1;}
                _ => {i += d * (c as i32 -'0' as i32); d*=10;}
            }
        }
        println!("{:.3}",sum);
    }
    Ok(())
}
