use data_structures::linkedlist::LinkedList;
use data_structures::looplist::looplist;
// use data_structures::min_heap::MinHeap;
fn main() {
    let mut list = LinkedList::new();
    list.push_front(4);
    list.push_back(2);
    list.push_front(2);
    list.push_front(4);
    println!("{:?}", list);

    // Infinite loop
    looplist();


}