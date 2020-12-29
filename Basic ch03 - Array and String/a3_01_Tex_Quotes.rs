use std::io::{self, Read};
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    let mut q = true;
    for byte in io::stdin().lock().bytes() {
        let c = byte? as char;
        if c=='"' {
            print!("{}",if q {"``"} else {"''"});
            q = !q;
        } else {
            print!("{}",c);
        }
    }
    Ok(())
}
