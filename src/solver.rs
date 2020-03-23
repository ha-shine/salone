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
#[derive(Copy, Clone, Eq, PartialEq)]
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

type CharSet = HashSet<char>;

pub struct Solver {
    graph: Graph,

    // dimensions and state of the board
    rows: usize,
    cols: usize,
    board: Vec<Option<TileLetter>>,

    // Sets of characters allowed on the given tile
    // None means the square is probably empty (or already have tile played which can be checked)
    // An empty set means there is no playable letter
    cross_sets: Vec<Option<CharSet>>,

    // List of index of anchors
    // Anchors are a set of tiles we can start looking for a legal move
    anchors: HashSet<usize>
}

impl Solver {
    pub fn new(rows: usize, cols: usize) -> Result<Self, &'static str> {
        if rows % 2 == 0 || cols % 2 == 0 {
            return Err("rows and cols must be odd numbers");
        }

        let mut solver = Solver {
            graph: Graph::new(),
            rows,
            cols,
            board: vec![None; rows * cols],
            cross_sets: vec![None; rows * cols],
            anchors: HashSet::new(),
        };

        // the center of the board is the only anchor at the start of the game
        let middle = solver.get_index(rows/2, cols/2);
        solver.anchors.insert(middle);
        solver.cross_sets[middle] = Some((b'a'..=b'z').map(char::from).collect());

        Ok(solver)
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