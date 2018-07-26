enum BinaryTrie {
    Node(Box<BinaryTrie>, Box<BinaryTrie>, bool),
    Empty,
}

impl BinaryTrie {
    fn new() -> Self {
        BinaryTrie::Empty
    }

    fn add<'a, T: Iterator<Item = bool>>(&mut self, mut iter: T) {
        use BinaryTrie::*;
        match iter.next() {
            Some(true) => match self {
                &mut Node(_, ref mut right, _) => {
                    right.add(iter);
                }
                &mut Empty => {
                    let mut r = Box::new(Empty);
                    r.add(iter);
                    *self = Node(Box::new(Empty), r, false);
                }
            },
            Some(false) => match self {
                &mut Node(ref mut left, _, _) => {
                    left.add(iter);
                }
                &mut Empty => {
                    let mut l = Box::new(Empty);
                    l.add(iter);
                    *self = Node(l, Box::new(Empty), false);
                }
            },
            None => match self {
                &mut Node(_, _, ref mut f) => *f = true,
                &mut Empty => *self = Node(Box::new(Empty), Box::new(Empty), true),
            },
        }
    }

    fn contain<T: Iterator<Item = bool>>(&mut self, mut iter: T) -> bool {
        use BinaryTrie::*;
        match iter.next() {
            Some(true) => match self {
                &mut Node(_, ref mut right, _) => right.contain(iter),
                &mut Empty => false,
            },
            Some(false) => match self {
                &mut Node(ref mut left, _, _) => left.contain(iter),
                &mut Empty => false,
            },
            None => match self {
                &mut Node(_, _, is_end) => is_end,
                &mut Empty => false,
            },
        }
    }
}
