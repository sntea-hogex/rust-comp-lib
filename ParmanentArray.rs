use std::rc::Rc;
// unverified
#[derive(Eq, PartialEq, Clone, Debug)]
enum ParsistentArray<T>
where
    T: std::clone::Clone + std::fmt::Debug,
{
    Node(usize, Rc<ParsistentArray<T>>, Rc<ParsistentArray<T>>),
    Leaf(T),
}

impl<T> ParsistentArray<T>
where
    T: std::clone::Clone + std::fmt::Debug,
{
    pub fn new(n: usize, init: T) -> Self {
        let mut l = 1;
        while l < n {
            l *= 2;
        }
        Rc::try_unwrap(Self::new_(l, init)).unwrap()
    }

    fn new_(n: usize, init: T) -> Rc<Self> {
        if n == 1 {
            Rc::new(ParsistentArray::Leaf(init.clone()))
        } else {
            Rc::new(ParsistentArray::Node(
                n,
                Self::new_(n / 2, init.clone()),
                Self::new_(n / 2, init),
            ))
        }
    }

    pub fn len(&self) -> usize {
        match self {
            &ParsistentArray::Leaf(_) => 1,
            &ParsistentArray::Node(n, _, _) => n,
        }
    }

    pub fn update(&self, i: usize, val: T) -> Self {
        debug_assert!(i < self.len());
        match self {
            &ParsistentArray::Leaf(_) => ParsistentArray::Leaf(val),
            &ParsistentArray::Node(n, ref left, ref right) => if i < n / 2 {
                ParsistentArray::Node(n, Rc::new(left.update(i, val)), right.clone())
            } else {
                ParsistentArray::Node(n, left.clone(), Rc::new(right.update(i - n / 2, val)))
            },
        }
    }

    pub fn get(&self, i: usize) -> &T {
        debug_assert!(i < self.len());
        match self {
            &ParsistentArray::Leaf(ref val) => val,
            &ParsistentArray::Node(n, ref left, ref right) => if i < n / 2 {
                left.get(i)
            } else {
                right.get(i - n / 2)
            },
        }
    }
}
