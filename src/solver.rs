use std::collections::{BinaryHeap, HashSet};

use crate::dag::Graph;
use std::cmp::Ordering;

// Representation of letters on the rack
pub enum RackLetter {
    Blank,
    Char(char),
}

// Representation of letters on the tile of the board
// The blank on the tile must represent a character
#[derive(Eq, PartialEq)]
pub enum TileLetter {
    Blank(char),
    Char(char),
}

// A single placement of letter on a tile with 0-index row and column
#[derive(Eq, PartialEq)]
pub struct TilePlacement {
    letter: TileLetter,
    row: usize,
    col: usize,
}

// A solution contains the placement of letters and it's total score
#[derive(Eq)]
pub struct Solution {
    placement: Vec<TilePlacement>,
    score: usize,
}

impl Ord for Solution {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        self.score.eq(&other.score)
    }
}

pub struct Solver {
    graph: Graph,

    // dimensions and state of the board
    rows: usize,
    cols: usize,
    board: Vec<Option<TileLetter>>,

    // List of index of anchors
    // Anchors are a set of tiles we can start looking for a legal move
    anchors: HashSet<usize>
}

impl Solver {
    pub fn new(rows: usize, cols: usize) -> Self {
        // TODO: check pre-requisite, ie rows and cols must be odds so that we can find the center
        let mut solver = Solver {
            graph: Graph::new(),
            rows,
            cols,
            board: Vec::with_capacity(rows * cols),
            anchors: HashSet::new(),
        };

        // all the tiles on the board at the beginning are empty
        for _ in 0..rows * cols {
            solver.board.push(None);
        }

        // the center of the board is the only anchor at the start of the game
        solver.anchors.insert(solver.get_index(rows/2, cols/2));

        solver
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        (self.cols * row) + col
    }

    fn compute_anchors(&mut self, placements: &Vec<TilePlacement>) {
        for placement in placements {
            let row = placement.row;
            let col = placement.col;

            let index = self.get_index(row, col);
            self.anchors.remove(&index);

            // check the tiles surrounding the current placement
            // if those tiles are empty, they can be anchors for next move
            if row > 0 {
                let index = self.get_index(row - 1, col);
                if self.board[index].is_none() {
                    self.anchors.insert(index);
                }
            }

            if row < self.rows - 1 {
                let index = self.get_index(row + 1, col);
                if self.board[index].is_none() {
                    self.anchors.insert(index);
                }
            }

            if col > 0 {
                let index = self.get_index(row, col - 1);
                if self.board[index].is_none() {
                    self.anchors.insert(index);
                }
            }

            if col < self.cols - 1 {
                let index = self.get_index(row, col + 1);
                if self.board[index].is_none() {
                    self.anchors.insert(index);
                }
            }
        }
    }

    pub fn add_dictionary_word(&mut self, word: &str) {
        self.graph.add_word(word)
    }

    pub fn place_tiles(&mut self, placements: Vec<TilePlacement>) {}

    pub fn solve(&mut self, letters: &Vec<RackLetter>) -> BinaryHeap<Solution> {
        BinaryHeap::new()
    }
}