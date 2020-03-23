use std::collections::{HashSet, HashMap};
use std::iter::Peekable;
use std::borrow::BorrowMut;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    Char(char),
    Delim,
}

pub struct Arc {
    pub letter_set: HashSet<char>,
    pub next: Node,
}

impl Arc {
    fn new() -> Self {
        Arc {
            letter_set: HashSet::new(),
            next: Node::new(),
        }
    }

    fn add_word<'a>(&mut self, kind: &Kind, mut words: Peekable<impl Iterator<Item=&'a Kind>>) {
        let peek = words.peek();

        match (kind, peek) {
            (Kind::Char(ch), None) => {
                self.letter_set.insert(*ch);
            },
            _ => {
                self.next.add_word(words);
            }
        }
    }
}

pub struct Node {
    // Arcs going out from this node, associated with the letter
    pub arcs: HashMap<Kind, Arc>,
}

impl Node {
    fn new() -> Self {
        Node {
            arcs: HashMap::new(),
        }
    }

    fn add_word<'a>(&mut self, mut words: Peekable<impl Iterator<Item=&'a Kind>>) {
        let next = words.next().unwrap();

        if let None = self.arcs.get(next) {
            self.arcs.insert(*next, Arc::new());
        }

        let mut arc = self.arcs.get_mut(next).unwrap();
        arc.add_word(next, words);
    }
}

pub struct Graph {
    pub init: Arc
}

impl Graph {
    pub fn new() -> Self {
        let mut init = Arc::new();
        init.next = Node::new();

        Graph {
            init
        }
    }

    pub fn add_word(&mut self, word: &str) {
        for i in 1..word.len() + 1 {
            let (head, tail) = word.split_at(i);
            let head = head.chars().rev().collect::<String>();

            let mut vec = Vec::new();

            for c in head.chars() {
                vec.push(Kind::Char(c));
            }

            if !tail.is_empty() {
                vec.push(Kind::Delim);
            }

            for c in tail.chars() {
                vec.push(Kind::Char(c));
            }

            self.init.next.add_word(vec.iter().peekable());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dag::Graph;

    #[test]
    fn it_works() {
        let words = vec!["care"];
        let mut g = Graph::new();
        for word in words {
            g.add_word(word);
        }
    }
}
