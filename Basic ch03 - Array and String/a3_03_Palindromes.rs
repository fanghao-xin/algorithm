use std::io::{self,BufRead};
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    let rev = "A   3  HIL JM O   2TUVWXY51SE Z  8 ".chars().collect::<Vec<_>>();
    let msg = ["not a palindrome","a regular palindrome","a mirrored string","a mirrored palindrome"];
    for s in io::stdin().lock().lines().map(|x| x.unwrap()).filter(|x| x!="") {
        let (mut p,mut m) = (1usize,1usize);
        let chars = s.chars().collect::<Vec<_>>();
        let len = chars.len();
        for i in 0..len {
            let c = chars[i];
            if chars[i]!=chars[len-i-1] {p = 0;}
            if rev[c as usize - if c.is_ascii_alphabetic() {'A' as usize} else {'0' as usize - 25}]!=chars[len-i-1] {m = 0;}
        }
        print!("{} -- is {}.\n\n",s,msg[(m<<1)+p]);
    }
    Ok(())
}
