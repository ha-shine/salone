enum Kind {
    Char(char),
    Blank(char)
}

struct Square {
    kind: Option<Kind>
}

impl Square {

    fn new() -> Self {
        Square {
            kind: None
        }
    }

}

pub struct Board {
    rows: usize,
    cols: usize,
    squares: Vec<Square>
}

impl Board {

    pub fn new(rows: usize, cols: usize) -> Self {
        let mut squares = Vec::with_capacity(rows*cols);
        for _ in 0..rows*cols {
            squares.push(Square::new());
        }

        Board {
            rows,
            cols,
            squares
        }
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        (self.cols * row) + col
    }

}