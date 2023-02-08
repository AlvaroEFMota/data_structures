use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
struct List {
    item: i32,
    next: Option<Rc<RefCell<List>>>,
}

pub fn LoopList() {
    let a = Rc::new(RefCell::new(List{ item: 1, next: None } ));
    let b = Rc::new(RefCell::new(List{ item: 2, next: None } ));
    let c = Rc::new(RefCell::new(List{ item: 3, next: None } ));

    {
        let mut a1 = a.as_ref().borrow_mut();
        a1.next = Some(Rc::clone(&b));
    }

    {
        let mut b1 = b.as_ref().borrow_mut();
        b1.next = Some(Rc::clone(&c));
    }

    {
        let mut c1 = c.as_ref().borrow_mut();
        c1.next = Some(Rc::clone(&a));
    }

    { // Infinite loop   
        let mut node = a.clone();
        loop {
            let a = Rc::clone(&node);
            let a_ref = a.as_ref().borrow();
            print!("{} ", a_ref.item);
            if let Some(_) = &a_ref.next {
                node = a_ref.next.clone().unwrap();
            } else { // just for remove the warning "unreachable statement"
                break
            }
        }
    }

    // fatal runtime error: stack overflow
    println!("{:?}", a);

}