use std::collections::{HashSet, HashMap};
use std::iter::Peekable;
use std::borrow::BorrowMut;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Kind {
    Char(char),
    Delim,
}

struct Node {
    // The set of letters which, if encountered next, make a word
    letter_set: HashSet<char>,

    // Arcs going out from this node, associated with the letter
    arcs: HashMap<Kind, Node>,
}

impl Node {
    fn new() -> Self {
        Node {
            letter_set: HashSet::new(),
            arcs: HashMap::new(),
        }
    }

    fn add_word<'a>(&mut self, mut words: Peekable<impl Iterator<Item=&'a Kind>>) {
        let next = words.next().unwrap();
        let peek = words.peek();

        match peek {
            None => {
                self.add_final(next)
            }

            Some(_) => {
                if let None = self.arcs.get(next) {
                    self.arcs.insert(*next, Self::new());
                }

                let mut next_state = self.arcs.get_mut(next).unwrap();
                next_state.add_word(words);
            }
        }
    }

    fn add_final(&mut self, kind: &Kind) {
        match kind {
            Kind::Char(c) => { self.letter_set.insert(*c); }
            _ => {}
        }
    }
}

pub struct Graph {
    root: Node
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            root: Node::new()
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

            self.root.add_word(vec.iter().peekable());
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
