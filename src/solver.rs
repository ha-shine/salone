use std::collections::{BinaryHeap, HashSet};

use crate::dag::{Graph, Arc, Kind};
use std::cmp::Ordering;

// Representation of letters on the rack
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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

impl TileLetter {
    fn to_char(&self) -> char {
        match self {
            TileLetter::Blank(ch) => *ch,
            TileLetter::Char(ch) => *ch
        }
    }
}

// A single placement of letter on a tile with 0-index row and column
#[derive(Copy, Clone, Eq, PartialEq)]
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

#[derive(Eq, PartialEq)]
enum Direction {
    TD,
    LR,
}

type CharSet = HashSet<char>;
type Pos = (usize, usize);

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
    anchors: HashSet<Pos>,
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
        solver.anchors.insert((rows / 2, cols / 2));

        let middle = solver.get_index(rows / 2, cols / 2);
        solver.cross_sets[middle] = Some((b'a'..=b'z').map(char::from).collect());

        Ok(solver)
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        (self.cols * row) + col
    }

    // TODO: cross set probably should be computed here too
    fn compute_anchors(&mut self, placements: &Vec<TilePlacement>) {
        for placement in placements {
            let row = placement.row;
            let col = placement.col;
            let pos = (row, col);

            self.anchors.remove(&pos);

            // check the tiles surrounding the current placement
            // if those tiles are empty, they can be anchors for next move
            if row > 0 {
                self.anchors.insert((row - 1, col));
            }

            if row < self.rows - 1 {
                self.anchors.insert((row + 1, col));
            }

            if col > 0 {
                self.anchors.insert((row, col - 1));
            }

            if col < self.cols - 1 {
                self.anchors.insert((row, col + 1));
            }
        }
    }

    pub fn add_dictionary_word(&mut self, word: &str) {
        self.graph.add_word(word)
    }

    pub fn place_tiles(&mut self, placements: Vec<TilePlacement>) {}

    pub fn solve(&mut self, letters: &Vec<RackLetter>) -> BinaryHeap<Solution> {
        let mut solutions = BinaryHeap::new();
        for anchor in &self.anchors {
            let mut placements = MoveGenerator::generate_moves(&self, letters, anchor.0, anchor.1, Direction::LR);
            for placement in placements {
                solutions.push(Solution {
                    placement,
                    score: 0,
                })
            }

            placements = MoveGenerator::generate_moves(&self, letters, anchor.0, anchor.1, Direction::TD);
            for placement in placements {
                solutions.push(Solution {
                    placement,
                    score: 0,
                })
            }
        }

        solutions
    }
}

struct MoveGenerator<'a> {
    solver: &'a Solver,
    row: usize,
    col: usize,
    dir: Direction,
    moves: Vec<Vec<TilePlacement>>
}

impl<'a> MoveGenerator<'a> {
    fn generate_moves(solver: &'a Solver, letters: &Vec<RackLetter>, row: usize, col: usize, dir: Direction) -> Vec<Vec<TilePlacement>> {
        let generator = MoveGenerator {
            solver,
            row,
            col,
            dir,
            moves: Vec::new(),
        };

        generator.moves
    }

    fn generate(&mut self, mut words: &Vec<TilePlacement>, offset: isize, letters: HashSet<RackLetter>, arc: &'a Arc) {
        let index = self.solver.get_index(self.row, self.col);
        if let Some(_) = self.solver.board[index] {
            // TODO: Go on
            return;
        }

        // no letter remaining, end function
        if letters.is_empty() {
            return;
        }

        let mut row = self.row as isize;
        let mut col = self.col as isize;
        match self.dir {
            Direction::TD => row += offset,
            Direction::LR => col += offset,
        };

        // this won't be out of index, the bound will be checked in go on method
        let index = self.solver.get_index(row as usize, col as usize);
        for letter in &letters {
            match letter {
                RackLetter::Char(ch) => {
                    if let Some(cross_set) = &self.solver.cross_sets[index] {
                        if cross_set.contains(ch) {
                            let mut cloned_letters = letters.clone();
                            cloned_letters.remove(letter);
                            // TODO: go on
                        }
                    }
                }
                RackLetter::Blank => {
                    if let Some(cross_set) = &self.solver.cross_sets[index] {
                        for playable_letter in cross_set {
                            // TODO: put blank into the word vector as playable_letter
                            //       and go on
                        }
                    }
                }
            }
        }
    }

    // TODO: should words be a linked list since we need to add both from front and back?
    fn go_on(&mut self, letter: TileLetter, words: &mut Vec<TilePlacement>,
             offset: isize, letters: HashSet<RackLetter>, old_arc: &'a Arc, mut new_arc: Option<&'a Arc>) {

        let mut row = self.row as isize;
        let mut col = self.col as isize;
        match self.dir {
            Direction::LR => col += offset,
            Direction::TD => row += offset
        };

        // moving left since offset is less than 0
        if offset <= 0 {
            words.insert(0, TilePlacement {
                letter,
                row: row as usize,
                col: col as usize
            });

            // if we have empty space on left and letter is an ending character, record play
            let empty_left = match self.dir {
                Direction::TD => {
                    row > 0 && self.solver.board[self.solver.get_index(row as usize - 1, col as usize)].is_none()
                },
                Direction::LR => {
                    col > 0 && self.solver.board[self.solver.get_index(row as usize, col as usize - 1)].is_none()
                }
            };
            if old_arc.letter_set.contains(&letter.to_char()) && empty_left {
                self.moves.push(words.clone());
            }

            if let Some(arc) = new_arc.take() {
                if (self.dir == Direction::LR && col > 0) || (self.dir == Direction::TD && row > 0) {
                    self.generate(words, offset - 1, letters.clone(), arc);
                }

                let new_arc = arc.next.arcs.get(&Kind::Delim);
                if new_arc.is_some() {
                    self.generate(words, 1, letters, new_arc.unwrap());
                }
            }

            return;
        } else {
            // in this place, offset is > 0, so we are moving right
            words.push(TilePlacement {
                letter,
                row: row as usize,
                col: col as usize,
            });

            // if we have empty space on the right and letter is an ending character, record play
            let empty_right = match self.dir {
                Direction::TD => {
                    row as usize + 1 < self.solver.rows
                        && self.solver.board[self.solver.get_index(row as usize + 1, col as usize)].is_none()
                },
                Direction::LR => {
                    col as usize + 1 < self.solver.cols
                        && self.solver.board[self.solver.get_index(row as usize, col as usize + 1)].is_none()
                }
            };
            if old_arc.letter_set.contains(&letter.to_char()) && empty_right {
                self.moves.push(words.clone());
            }
        }
    }
}