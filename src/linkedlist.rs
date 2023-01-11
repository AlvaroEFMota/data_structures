use std::borrow::BorrowMut;

#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
       LinkedList(None) 
    }

    pub fn push_front(&mut self, item: T) {
        let current = self.0.take();
        self.0 = Some((item, Box::new(LinkedList(current))));
    }

    pub fn push_back(&mut self, item: T) {
        let mut node_it = self;
        loop {
            if let Some(_) = &node_it.0 { // using the reference to check if it is Some
                node_it = node_it.0.as_mut().unwrap().1.borrow_mut();
            } else {
                node_it.0 = Some((item, Box::new(LinkedList(None))));
                break;
            }
        }
    }

    pub fn push_back_recursive(&mut self, item: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(item),
            None => self.0 = Some((item, Box::new(LinkedList(None))))
        }
    }

    // Challange: insert sorted
}

#[cfg(test)]
mod test {

    #[test]
    fn compiled() {
        assert_eq!(1,1);
    }
}