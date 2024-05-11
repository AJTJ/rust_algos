// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// pub fn add_two_numbers(
//     l1: Option<Box<ListNode>>,
//     l2: Option<Box<ListNode>>,
// ) -> Option<Box<ListNode>> {
// }

fn push_front(current_list: ListNode, new_value: i32) -> ListNode {
    let new_node = ListNode {
        val: new_value,
        next: Some(Box::new(current_list.clone())),
    };
    new_node
}

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

fn main() {
    let list_a1 = ListNode::new(1);

    let list_a2 = push_front(list_a1, 2);
    let list_a3 = push_front(list_a2, 3);

    let list_a = push_front(list_a3, 4);

    let list_b1 = ListNode::new(4);

    let list_b2 = push_front(list_b1, 5);

    let list_b = push_front(list_b2, 6);

    println!("a: `{:?}", list_a);
    println!("b: {:?}", list_b);

    fn create_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut num_vec = Vec::new();
        fn unravel(list: Option<Box<ListNode>>, num_vec: &mut Vec<i32>) -> Vec<i32> {
            if let Some(list) = list {
                num_vec.push(list.val);
                if let Some(_) = list.next {
                    unravel(list.next, num_vec)
                } else {
                    num_vec.clone()
                }
            } else {
                num_vec.clone()
            }
        }
        unravel(list, &mut num_vec);
        num_vec
    }

    println!("a: {:?}", create_vec(Some(Box::new(list_a))));
    println!("b: {:?}", create_vec(Some(Box::new(list_b))));

    // fn add_two_numbers(
    //     l1: Option<Box<ListNode>>,
    //     l2: Option<Box<ListNode>>,
    // ) -> Option<Box<ListNode>> {
    // }
}
