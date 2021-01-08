use std::io::{self,BufRead};
use std::error::Error;

#[inline]
fn is_right(a: &[i32], b: &[i32]) ->bool {
    a.iter().zip(b).fold(true, |f,(a,b)| f && a+b<=3)
}

fn main() -> Result<(),Box<dyn Error>> {
    let mut a = Vec::new();
    for line in io::stdin().lock().lines().map(|x| x.unwrap()) {
        if a.len()==0 {
            a = line.chars().map(|x| x as i32 - '0' as i32).collect();
        } else {
            let mut line:Vec<_> = line.chars().map(|x| x as i32 - '0' as i32).collect();
            if line.len() < a.len() {let t=a; a=line; line=t}
            if (0..=line.len()-a.len()).fold(false, |f, i| { f || is_right(&line[i..i+a.len()], &a[..]) }) {
                println!("{}",line.len());
            } else {
                for i in (0..a.len()).rev() {
                    if is_right(&line[..i], &a[a.len()-i..])
                    || is_right(&line[line.len()-i..], &a[..i])
                    {
                        println!("{}",line.len()+a.len()-i);
                        break;
                    }
                }
            }
            a.clear();
        }
    }
    Ok(())
}
