use std::io;
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    let mut buf = String::new();
    let mut game = 0;

    loop {
        buf.clear();
        io::stdin().read_line(&mut buf)?;
        let n = buf.trim().parse::<i32>()?;
        if n==0 {break;}

        println!("Game {}:",{game+=1;game});
        buf.clear();
        io::stdin().read_line(&mut buf)?;
            let ans = buf
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

        loop {
            buf.clear();
            io::stdin().read_line(&mut buf)?;
            if &buf[0..1]=="0" {break;}

            let mut same_count = 0;
            let mut ans_count = [0;10];
            let mut guess_count = [0;10];

            buf
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .zip(&ans)
                .for_each(|(g,&a)| {
                    ans_count[a as usize] += 1;
                    guess_count[g as usize] += 1;
                    if g==a {same_count+=1;}
                });

            let both_count = ans_count.iter().zip(&guess_count).fold(0,|sum,(x,y)| {
                sum + if x<y {x} else {y}
            });
            
            println!("\t({},{})",same_count,both_count-same_count);

        }
    }
    Ok(())
}
