use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
pub struct List {
    pub item: i32,
    pub next: Option<Rc<RefCell<List>>>,
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

    // Uncomment the line below to see a stack overflow
    //looplist_recursive_stackoverflow(&Some(a.clone())); 

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

pub fn looplist_recursive_stackoverflow(node_option:  &Option<Rc<RefCell<List>>>) {
    if let Some(_) = node_option {
        let x = node_option.clone().unwrap();
        let y = x.borrow();
        print!("{} ", y.item);
        looplist_recursive_stackoverflow(&y.next);
    }
}