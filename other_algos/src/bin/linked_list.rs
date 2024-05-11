use std::collections::LinkedList;

fn main() {
    let mut my_list: LinkedList<u32> = LinkedList::new();

    my_list.push_front(1);
    my_list.push_front(2);
    my_list.push_front(3);

    println!("{:?}", my_list);
}
