#[derive(Clone, Debug)]
enum List<T> {
    Cons(T, std::rc::Rc<List<T>>), 
    Nil,
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = ListRefIterator<'a, T>;
    fn into_iter(self) -> ListRefIterator<'a, T> {
        ListRefIterator { list: self }
    }
}

struct ListRefIterator<'a, T: 'a> {
    list: &'a List<T>,
}

impl<'a, T> Iterator for ListRefIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match *self.list {
            List::Cons(ref val, ref next) => {
                *self = next.into_iter();
                Some(val)
            },
            List::Nil => {
                None
            },
        }
    }
}



impl<T> List<T>
where
    T: std::fmt::Display
{
    fn iter<'a>(&'a self) -> ListRefIterator<'a, T> {
        self.into_iter()
    }

    fn print(&self) {
        match self {
            &List::Cons(ref v, ref nex) => {
                print!("{} ", v);
                nex.print();
            },
            &List::Nil => {
                // println!("");
            },
        } 
    }
}

macro_rules! list {
    () => {
        List::Nil
    };
    ($h: expr, $($t: expr,)*) => {
        List::Cons($h, std::rc::Rc::new(list!($($t,)*)))
    };
}
