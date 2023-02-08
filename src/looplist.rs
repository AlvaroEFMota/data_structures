use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
struct List {
    item: i32,
    next: Option<Rc<RefCell<List>>>,
}

#[allow(dead_code)]
pub fn looplist() {
    let a = Rc::new(RefCell::new(List{ item: 1, next: None } ));
    let b = Rc::new(RefCell::new(List{ item: 2, next: None } ));
    let c = Rc::new(RefCell::new(List{ item: 3, next: None } ));

    c.borrow_mut().item = 7;

    {
        // let mut a1 = a.borrow_mut();
        let mut a1 = a.borrow_mut();
        a1.next = Some(Rc::clone(&b));
    }

    {
        let mut b1 = b.borrow_mut();
        b1.next = Some(Rc::clone(&c));
    }

    {
        let mut c1 = c.borrow_mut();
        c1.next = Some(Rc::clone(&a));
    }

    { // Infinite loop   
        let mut node = a.clone();
        loop {
            let a = Rc::clone(&node);
            let a_ref = a.borrow();
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