// Creation of a min heap
// a max heap is the same thing but opposite

// GET INDEX
fn get_left_child_index(parent_index: usize) -> usize {
    2 * parent_index + 1
}
fn get_right_child_index(parent_index: usize) -> usize {
    2 * parent_index + 2
}
fn get_parent_index(child_index: usize) -> Option<usize> {
    match child_index.checked_sub(1) {
        Some(x) => x.checked_div(2),
        None => None,
    }
}

// HAS RELATIONS
fn has_left_child(parent_index: usize, size: usize) -> bool {
    get_left_child_index(parent_index) < size
}
fn has_right_child(parent_index: usize, size: usize) -> bool {
    get_right_child_index(parent_index) < size
}
fn has_parent(el_index: usize) -> bool {
    match get_parent_index(el_index) {
        Some(_) => true,
        _ => false,
    }
}

// GET VAL

fn left_child(parent_index: usize, items: &Vec<i32>) -> i32 {
    items[get_left_child_index(parent_index)]
}
fn right_child(parent_index: usize, items: &Vec<i32>) -> Option<i32> {
    items.get(get_right_child_index(parent_index)).copied()
}
fn parent(el_index: usize, items: &Vec<i32>) -> Option<i32> {
    if let Some(par_i) = get_parent_index(el_index) {
        Some(items[par_i])
    } else {
        None
    }
}

fn swap(index1: usize, index2: usize, items: &mut Vec<i32>) {
    let temp = items[index1];
    items[index1] = items[index2];
    items[index2] = temp;
}

fn ensure_extra_capacity(capacity: &mut usize, items: &mut Vec<i32>, size: &mut usize) {
    if size == capacity {
        *capacity *= 2;
        let mut new_items: Vec<i32> = Vec::with_capacity(*capacity);
        new_items.append(items);
        *items = new_items;
    }
}

fn peek(items: Vec<i32>) -> Option<i32> {
    items.get(0).copied()
}

fn poll(items: &mut Vec<i32>, size: &mut usize) -> i32 {
    let item = items[0];
    items[0] = items.pop().unwrap();
    *size = *size - 1;
    heapify_down(size, items);
    item
}

fn heapify_down(size: &usize, items: &mut Vec<i32>) {
    let mut index = 0;
    while has_left_child(index, *size) {
        let mut smaller_child_index = get_left_child_index(index);
        if let Some(r_val) = right_child(index, items) {
            if r_val < left_child(index, items) {
                smaller_child_index = get_right_child_index(index);
            }
        }
        if items[index] < items[smaller_child_index] {
            break;
        } else {
            swap(index, smaller_child_index, items);
        }
        index = smaller_child_index;
    }
}

fn add(item: i32, capacity: &mut usize, items: &mut Vec<i32>, size: &mut usize) {
    ensure_extra_capacity(capacity, items, size);
    items.push(item);
    *size = *size + 1;
    heapify_up(items, size);
}

fn heapify_up(items: &mut Vec<i32>, size: &mut usize) {
    let mut i = *size - 1;

    while has_parent(i) && (parent(i, &items) > Some(items[i])) {
        println!(
            "IN HEAP UP - items: {:?}, the size: {}, the i: {}, the el: {}, the parent: {:?}",
            items,
            size,
            i,
            items[i],
            parent(i, items)
        );
        swap(get_parent_index(i).unwrap(), i, items);
        if let Some(par_i) = get_parent_index(i) {
            i = par_i
        }
    }
}

fn main() {
    let mut capacity = 3;
    let mut items: Vec<i32> = Vec::with_capacity(capacity);
    let mut size = 0;

    // println!("items: {:?}, capacity: {}, size {}", items, capacity, size);
    // add(4, &mut capacity, &mut items, &mut size);
    // println!("items: {:?}, capacity: {}, size {}", items, capacity, size);
    // add(7, &mut capacity, &mut items, &mut size);
    // println!("items: {:?}, capacity: {}, size {}", items, capacity, size);
    // add(3, &mut capacity, &mut items, &mut size);
    // println!("items: {:?}, capacity: {}, size {}", items, capacity, size);
    // add(6, &mut capacity, &mut items, &mut size);
    // println!("items: {:?}, capacity: {}, size {}", items, capacity, size);
    // add(5, &mut capacity, &mut items, &mut size);
    // println!("items: {:?}, capacity: {}, size {}", items, capacity, size);
    // add(2, &mut capacity, &mut items, &mut size);
    // println!("items: {:?}, capacity: {}, size {}", items, capacity, size);
    // add(1, &mut capacity, &mut items, &mut size);
    // println!("items: {:?}, capacity: {}, size {}", items, capacity, size);

    // println!("IS OTHER NOW");

    let mut other_capacity = 10;
    let mut other_items: Vec<i32> = Vec::with_capacity(other_capacity);
    let mut these_items: Vec<i32> = vec![10, 15, 20, 17, 25];
    let mut other_size = 5;
    other_items.append(&mut these_items);

    poll(&mut other_items, &mut other_size);
    // add(8, &mut other_capacity, &mut other_items, &mut other_size);
    println!(
        "Other items: {:?}, Other size: {:?}",
        other_items, other_size
    );
}
