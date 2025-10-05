#[derive(Debug, PartialEq)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    #[inline]
    fn head(&self) -> Option<&i32> {
        match self {
            List::Cons(v, _) => Some(v),
            List::Nil => None,
        }
    }

    #[inline]
    fn tail(&self) -> Option<&List> {
        match self {
            List::Cons(_, next) => Some(next),
            List::Nil => None,
        }
    }

    /// Borrowing iterator
    fn iter(&self) -> Iter<'_> {
        Iter { next: Some(self) }
    }
}

impl From<Vec<i32>> for List {
    fn from(vec: Vec<i32>) -> List {
        vec.into_iter()
            .rev()
            .fold(List::Nil, |current, val| List::Cons(val, Box::new(current)))
    }
}

/// Borrowing iterator over List values
struct Iter<'a> {
    next: Option<&'a List>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.take()? {
            List::Cons(v, next) => {
                self.next = Some(next);
                Some(*v)
            }
            List::Nil => None,
        }
    }
}

fn main() {
    let li = List::from(vec![1, 23, 4, 5345, 67, 456, 3]);
    println!("FIRST NUMBER: {}", li.head().unwrap());
    println!("SECOND NUMBER: {}", li.tail().unwrap().head().unwrap());

    for (i, val) in li.iter().enumerate() {
        println!("{}th number in list: {}", i, val);
    }
}
