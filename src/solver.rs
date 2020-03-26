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
    // An empty set means there is no playable letter
    // 0: cross sets for left-right plays, 1: cross sets for top-down plays
    cross_sets: (Vec<CharSet>, Vec<CharSet>),

    // List of index of anchors
    // Anchors are a set of tiles we can start looking for a legal move
    anchors: HashSet<Pos>,
}

impl Solver {
    pub fn new(rows: usize, cols: usize) -> Result<Self, &'static str> {
        if rows % 2 == 0 || cols % 2 == 0 {
            return Err("rows and cols must be odd numbers");
        }

        let mut lr_cross = Vec::new();
        let mut td_cross = Vec::new();
        let charset = (b'a'..=b'z').map(char::from).collect::<HashSet<_>>();

        for _ in 0..rows*cols {
            lr_cross.push(charset.clone());
            td_cross.push(charset.clone());
        }

        let mut solver = Solver {
            graph: Graph::new(),
            rows,
            cols,
            board: vec![None; rows * cols],
            cross_sets: (lr_cross, td_cross),
            anchors: HashSet::new(),
        };

        // the center of the board is the only anchor at the start of the game
        solver.anchors.insert((rows / 2, cols / 2));

        Ok(solver)
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        (self.cols * row) + col
    }

    fn get_cross_set(&self, row: usize, col: usize, dir: &Direction) -> &CharSet {
        let i = self.get_index(row, col);
        match dir {
            Direction::LR => &self.cross_sets.0[i],
            Direction::TD => &self.cross_sets.1[i]
        }
    }

    fn get_left_most_anchor(&self, pos: &Pos, dir: &Direction) -> Pos {
        match dir {
            Direction::LR => {
                let (mut row, mut col) = *pos;
                while row > 0 {
                    if self.anchors.contains(&(row - 1, col)) {
                        row -= 1;
                    }
                }

                return (row, col)
            },
            Direction::TD => {
                let (mut row, mut col) = *pos;
                while col > 0 {
                    if self.anchors.contains(&(row, col - 1)) {
                        row -= 1;
                    }
                }

                return (row, col)
            }
        }
    }

    // TODO: cross set probably should be computed here too
    fn compute_anchors_and_cross_set(&mut self, placements: &Vec<TilePlacement>) {
        let mut new_anchors = Vec::new();

        for placement in placements {
            let row = placement.row;
            let col = placement.col;
            let pos = (row, col);
            new_anchors.push(pos);

            // empty the cross sets for this index
            let i = self.get_index(row, col);
            self.cross_sets.0[i].clear();
            self.cross_sets.1[i].clear();

            // check the tiles surrounding the current placement
            // if those tiles are empty, they can be anchors for next move
            if row > 0 && self.board[self.get_index(row - 1, col)].is_none() {
                self.anchors.insert((row - 1, col));
            }

            if row < self.rows - 1 && self.board[self.get_index(row + 1, col)].is_none() {
                self.anchors.insert((row + 1, col));
            }

            if col > 0 && self.board[self.get_index(row, col - 1)].is_none() {
                self.anchors.insert((row, col - 1));
            }

            if col < self.cols - 1 && self.board[self.get_index(row, col + 1)].is_none() {
                self.anchors.insert((row, col + 1));
            }
        }

        for anchor in new_anchors {
            self.anchors.remove(&anchor);
        }
    }

    pub fn add_dictionary_word(&mut self, word: &str) {
        self.graph.add_word(word)
    }

    pub fn place_tiles(&mut self, placements: Vec<TilePlacement>) {}

    pub fn solve(&mut self, letters: &Vec<RackLetter>) -> BinaryHeap<Solution> {
        let mut solutions = BinaryHeap::new();
        let anchors = self.anchors.clone();

        for anchor in anchors {
            let left_anchor = self.get_left_most_anchor(&anchor, &Direction::LR);
            if self.anchors.contains(&left_anchor) {
                self.generate_moves_in_dir(letters, &left_anchor, Direction::LR, &mut solutions);
            }

            let left_anchor = self.get_left_most_anchor(&anchor, &Direction::LR);
            if self.anchors.contains(&left_anchor) {
                self.generate_moves_in_dir(letters, &left_anchor, Direction::LR, &mut solutions);
            }
        }

        solutions
    }

    fn generate_moves_in_dir(&mut self,
                             letters: &Vec<RackLetter>,
                             anchor: &Pos,
                             dir: Direction,
                             solutions: &mut BinaryHeap<Solution>) {
        let mut tiles = MoveGenerator::generate_moves(&self, letters, anchor.0, anchor.1, dir);
        for placements in tiles {
            for placement in &placements {
                self.anchors.remove(&(placement.row, placement.col));
            }

            // TODO: Calculate score
            solutions.push(Solution {
                placement: placements,
                score: 0,
            })
        }
    }
}

struct MoveGenerator<'a> {
    solver: &'a Solver,
    row: usize,
    col: usize,
    dir: Direction,
    moves: Vec<Vec<TilePlacement>>,
}

impl<'a> MoveGenerator<'a> {
    fn generate_moves(solver: &'a Solver, rack: &Vec<RackLetter>, row: usize, col: usize, dir: Direction) -> Vec<Vec<TilePlacement>> {
        let mut generator = MoveGenerator {
            solver,
            row,
            col,
            dir,
            moves: Vec::new(),
        };
        generator.generate(&mut Vec::new(), 0, rack.clone(), &generator.solver.graph.init);

        generator.moves
    }

    fn generate(&mut self, words: &mut Vec<TilePlacement>, offset: isize, rack: Vec<RackLetter>, arc: &'a Arc) {
        let index = self.solver.get_index(self.row, self.col);
        if let Some(tile) = self.solver.board[index] {
            self.go_on(tile, words, offset, rack, arc, arc.next.arcs.get(&Kind::Char(tile.to_char())));
            return;
        }

        // no letter remaining, end function
        if rack.is_empty() {
            return;
        }

        let mut row = self.row as isize;
        let mut col = self.col as isize;
        match self.dir {
            Direction::TD => row += offset,
            Direction::LR => col += offset,
        };

        // this won't be out of index, the bound will be checked in go on method
        let cross_set = self.solver.get_cross_set(row as usize, col as usize, &self.dir);
        if cross_set.is_empty() {
            return; // no letter eligible here, return early
        }

        for (idx, letter) in (&rack).iter().enumerate() {
            match letter {
                RackLetter::Char(ch) if cross_set.contains(ch) => {
                    let new_arc = arc.next.arcs.get(&Kind::Char(*ch));
                    let mut new_rack = rack.clone();
                    new_rack.remove(idx);

                    self.go_on(TileLetter::Char(*ch), words, offset, new_rack, arc, new_arc);
                }
                RackLetter::Blank => {
                    for playable in cross_set {
                        let new_arc = arc.next.arcs.get(&Kind::Char(*playable));
                        let mut new_rack = rack.clone();
                        new_rack.remove(idx);

                        self.go_on(TileLetter::Blank(*playable), words, offset, new_rack, arc, new_arc);
                    }
                }
                _ => { /* do nothing */ }
            }
        }
    }

    // TODO: should words be a linked list since we need to add both from front and back?
    fn go_on(&mut self, letter: TileLetter, placements: &mut Vec<TilePlacement>,
             offset: isize, rack: Vec<RackLetter>, old_arc: &'a Arc, mut new_arc: Option<&'a Arc>) {

        let mut row = self.row as isize;
        let mut col = self.col as isize;
        match self.dir {
            Direction::LR => col += offset,
            Direction::TD => row += offset
        };

        // moving left since offset is less than 0
        if offset <= 0 {
            placements.insert(0, TilePlacement {
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
                self.moves.push(placements.clone());
            }

            if let Some(arc) = new_arc.take() {
                if (self.dir == Direction::LR && col > 0) || (self.dir == Direction::TD && row > 0) {
                    self.generate(placements, offset - 1, rack.clone(), arc);
                }

                let new_arc = arc.next.arcs.get(&Kind::Delim);
                if new_arc.is_some() {
                    self.generate(placements, 1, rack, new_arc.unwrap());
                }
            }
        } else {
            // in this place, offset is > 0, so we are moving right
            placements.push(TilePlacement {
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
                self.moves.push(placements.clone());
            }

            if (self.dir == Direction::LR && col < self.solver.cols as isize - 1) ||
                (self.dir == Direction::TD && row < self.solver.cols as isize - 1) {
                if let Some(arc) = new_arc {
                    self.generate(placements, offset + 1, rack, arc);
                }
            }
        }
    }
}