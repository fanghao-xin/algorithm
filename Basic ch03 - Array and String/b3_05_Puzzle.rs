use std::io::{self,BufRead};
use std::error::Error;
use std::collections::BTreeMap;

fn main() -> Result<(),Box<dyn Error>> {
    const FRAME_SIZE_X:usize = 5;const FRAME_SIZE_Y:usize = 5;
    let cmd_pos:BTreeMap<_,_> ="ABLR".chars().zip(&[(-1,0),(1,0),(0,-1),(0,1)]).collect();
    let mut puzzle_id = 0;
    let mut cmd = String::new();
    let (mut x,mut y) = (0,0);
    let mut frame_filled = 0;
    let mut frame = [[' ';FRAME_SIZE_Y];FRAME_SIZE_X];
    for line in io::stdin().lock().lines().map(|x| x.unwrap()) {
        if frame_filled<FRAME_SIZE_X {
            line.chars().zip(0..FRAME_SIZE_Y).for_each(|(c,y)| frame[frame_filled][y]=c);
            if let Some(i)=frame[frame_filled].iter().position(|&x| x==' ') { x = frame_filled; y = i; }
            frame_filled += 1;
        } else if &line[line.len()-1..]!="0" {
            cmd += &line;
        } else {
            cmd += &line[..line.len()-1];
            println!("Puzzle #{}:",{puzzle_id += 1; puzzle_id});
            let mut fin = true;
            for c in cmd.chars() {
                match c {
                    'A'|'B'|'L'|'R' => {
                        let (xx,yy) = (x as i32+cmd_pos[&c].0,y as i32+cmd_pos[&c].1);
                        if 0<=xx && xx<FRAME_SIZE_X as i32 && 0<=yy && yy<FRAME_SIZE_Y as i32 {
                            let t = frame[x][y];
                            frame[x][y] = frame[xx as usize][yy as usize];
                            frame[xx as usize][yy as usize] = t;
                            x = xx as usize; y = yy as usize;
                        } else {println!("This puzzle has no final configuration.");fin=false;break;}
                    }
                    _ => {println!("This puzzle has no final configuration.");fin=false;break;}
                }
            }
            if fin {frame.iter().for_each(|x| {x.iter().for_each(|i| print!("{} ",i));println!();});}
            println!();
            cmd.clear(); x=0; y=0; frame_filled = 0;
            frame = [[' ';FRAME_SIZE_Y];FRAME_SIZE_X];
        }
    }
    Ok(())
}
