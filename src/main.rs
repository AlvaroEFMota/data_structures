use data_structures::linkedlist::LinkedList;
fn main() {
    let mut list = LinkedList::new();
    list.push_front(4);
    list.push_back(2);
    list.push_front(2);
    list.push_front(4);
    println!("{:?}", list);
}