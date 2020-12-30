use std::io::{self,Read};
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>>{
    let s = r"1234567890-=QWERTYUIOP[]\ASDFGHJKL;'ZXCVBNM,./";
    for byte in io::stdin().lock().bytes() {
        let c = byte? as char;
        if let Some(i) = s.find(c) {
            print!("{}",&s[i-1..i]);
        } else {
            print!("{}",c);
        }
    }
    Ok(())
}
