use crate::dag::Graph;
use crate::board::Board;

// Representation of letters on the rack
pub enum RackLetter {
    Blank,
    Char(char)
}

// Representation of letters on the tile of the board
// The blank on the tile must represent a character
pub enum TileLetter {
    Blank(char),
    Char(char)
}

// A single placement of letter on a tile with 0-index row and column
pub struct TilePlacement {
    letter: TileLetter,
    row: usize,
    col: usize
}

// A solution contains the placement of letters and it's total score
pub struct Solution {
    placement: Vec<TilePlacement>,
    score: usize,
}

pub struct Solver {
    graph: Graph,
    board: Board,
}

impl Solver {

    pub fn new(rows: usize, cols: usize) -> Self {
        Solver {
            graph: Graph::new(),
            board: Board::new(rows, cols)
        }
    }

    pub fn add_dictionary_word(&mut self, word: &str) {
        self.graph.add_word(word)
    }

    pub fn place_tiles(&mut self, placements: Vec<TilePlacement>) {

    }

    pub fn solve(&mut self, letters: &Vec<RackLetter>, limit: usize) -> Vec<Solution> {
        Vec::new()
    }

}