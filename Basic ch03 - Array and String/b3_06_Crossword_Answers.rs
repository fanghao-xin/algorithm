use std::io::{self,BufRead};
use std::error::Error;

#[derive(PartialEq,Copy,Clone)]
enum PuzzleDir { All, Across, Down }

struct Puzzle {
    id: i32,
    grid: Vec<Vec<char>>,
    ident: Vec<(usize,usize)>,
}

impl Puzzle {
    fn new(id:i32) -> Self { Puzzle{id, grid:Vec::new(), ident:vec![(0,0)]} }

    fn fill(&mut self, line:String) {
        self.grid.push(Vec::new());
        let i = self.grid.len()-1;
        line.char_indices().for_each(|(j,x)| {
            self.grid[i].push(x);
            if x!='*' && self.is_start_cell(i,j,PuzzleDir::All) { self.ident.push((i,j)) }
        });
    }

    fn is_start_cell(&self, i:usize, j:usize, dir:PuzzleDir) -> bool {
        if (dir!=PuzzleDir::Across && (i as i32 - 1 < 0 || self.grid[i-1][j] == '*'))
        || (dir!=PuzzleDir::Down && (j as i32 - 1 < 0 || self.grid[i][j-1] == '*'))
        { true } else { false }
    }

    fn print(&mut self, dir:PuzzleDir) {
        match dir {
            PuzzleDir::Across => println!("Across"),
            PuzzleDir::Down => println!("Down"),
            PuzzleDir::All => {
                self.print(PuzzleDir::Across);
                self.print(PuzzleDir::Down);
                return
            }
        }
        let (size_x, size_y) = (self.grid.len(), self.grid[0].len());
        for (n,(i,j)) in self.ident.iter().enumerate().skip(1) {
            if !self.is_start_cell(*i,*j,dir) { continue }
            print!("{}.",n);
            let (mut k, size) = if dir==PuzzleDir::Across { (*j, size_y) } else { (*i, size_x) };
            while k < size {
                let x = if dir==PuzzleDir::Across { self.grid[*i][k] } else { self.grid[k][*j] };
                if x=='*' { break } 
                print!("{}",x);
                k += 1;
            }
            println!();
        }
    }
}

fn main() -> Result<(),Box<dyn Error>> {
    let mut puzzle = Puzzle::new(0);
    for line in io::stdin().lock().lines().map(|x| x.unwrap()) {
        let c = line.chars().next().unwrap();
        if c.is_ascii_digit() {
            if puzzle.id > 0 {
                println!("Puzzle #{}:", puzzle.id);
                puzzle.print(PuzzleDir::All);
            }
            puzzle = Puzzle::new(puzzle.id+1);
            if c == '0' { break }
        }
        else { puzzle.fill(line) }
    }
    Ok(())
}
