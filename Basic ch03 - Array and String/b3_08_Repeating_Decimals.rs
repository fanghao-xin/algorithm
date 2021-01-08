use std::io::{self,BufRead};
use std::error::Error;
use std::collections::BTreeMap;

fn main() -> Result<(),Box<dyn Error>> {
    for line in io::stdin().lock().lines().map(|x| x.unwrap()) {
        let line = line.split_whitespace().collect::<Vec<_>>();
        let mut a:i32 = line[0].parse()?;
        let b:i32 = line[1].parse()?;
        let mut reminder:BTreeMap<i32,usize> = BTreeMap::new();
        let mut nums = String::with_capacity(50);
        let cycle_start;
        let mut cycle_end = 0;
        print!("{}/{} = {}.", a, b, a/b);
        let mut r = a%b;
        loop {
            if let Some(s)  = reminder.get(&r) {
                cycle_start = *s;
                break;
            } else {
                reminder.insert(r, cycle_end); cycle_end+=1;
                a = r*10;
                if nums.len() < 50 { nums.push((a / b +'0' as i32)as u8 as char) }
                r = a % b;
            }
        }
        println!("{}({}{})",&nums[..cycle_start], &nums[cycle_start..nums.len()], if cycle_end>50 {"..."} else {""});
        println!("{} = number of digits in repeating cycle\n",cycle_end-cycle_start);
    }
    Ok(())
}
