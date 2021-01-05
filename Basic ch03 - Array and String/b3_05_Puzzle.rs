use std::io::{self,BufRead};
use std::fmt;
use std::error::Error;
use std::collections::BTreeMap;

struct Puzzle {
    id: i32,
    grid: Vec<Vec<char>>,
    filled: usize,
    x:usize, y:usize,
    size_x: usize, size_y: usize
}

impl Puzzle {
    fn new(id:i32) -> Self { Puzzle::new_with_xy(id,5,5) }

    fn new_with_xy(id:i32,size_x:usize, size_y:usize) -> Self {
        Puzzle{id, grid:vec![vec![' ';size_x];size_y], filled:0, x:0, y:0, size_x, size_y}
    }

    fn fill(&mut self, line:String) {
        line.chars().zip(0..self.size_y).for_each(|(c,y)| self.grid[self.filled][y]=c);
        if let Some(i)=self.grid[self.filled].iter().position(|&x| x==' ') { self.x = self.filled; self.y = i; }
        self.filled += 1;
    }

    fn is_full(&self) -> bool { self.filled == self.size_x }

    fn mov(&mut self,p:(i32,i32)) -> bool {
        let (xx,yy) = (self.x as i64 + p.0 as i64, self.y as i64 + p.1 as i64);
        if 0<=xx && xx<self.size_x as i64 && 0<=yy && yy<self.size_y as i64 {
            let t = self.grid[self.x][self.y];
            self.grid[self.x][self.y] = self.grid[xx as usize][yy as usize];
            self.grid[xx as usize][yy as usize] = t;
            self.x = xx as usize; self.y = yy as usize;
            true
        } else { false }
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in &self.grid { for i in x { write!(f, "{} ",i)? } write!(f, "\n")? }
        Ok(())
    }
}

fn main() -> Result<(),Box<dyn Error>> {
    let cmd_pos:BTreeMap<_,_> ="ABLR".chars().zip(&[(-1,0),(1,0),(0,-1),(0,1)]).collect();
    let mut cmd = String::new(); let mut puzzle = Puzzle::new(1);
    for line in io::stdin().lock().lines().map(|x| x.unwrap()) {
        if !puzzle.is_full() { puzzle.fill(line) } 
        else if &line[line.len()-1..]!="0" { cmd += &line } 
        else {
            cmd += &line[..line.len()-1];
            println!("Puzzle #{}:",puzzle.id);
            let mut fin = true;
            for c in cmd.chars() {
                match c {
                    'A'|'B'|'L'|'R' => { if !puzzle.mov(*cmd_pos[&c]) {println!("This puzzle has no final configuration.\n"); fin=false; break } }
                    _ => {}
                }
            }
            if fin {println!("{}",puzzle)}
            cmd.clear(); puzzle = Puzzle::new(puzzle.id+1);
        }
    }
    Ok(())
}
