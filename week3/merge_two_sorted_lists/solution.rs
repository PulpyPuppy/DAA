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
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Option::<Box<ListNode>>::None;
        let mut next = &mut head;
        let mut list1 = list1;
        let mut list2 = list2;

        // I cried many times before the borrow-checker
        // accepted the code

        while list1.is_some() || list2.is_some() {
            match (&list1, &list2) {
                (rest, None) => {
                    *next = list1.take();
                    break;
                },
                (None, rest) => {
                    *next = list2.take();
                    break;
                },
                (Some(a), Some(b)) => {
                    if a.val < b.val {
                        *next = list1.take();
                        list1 = next.as_mut().unwrap().next.take();
                    } else {
                        *next = list2.take();
                        list2 = next.as_mut().unwrap().next.take();
                    }
                },
                (None, None) => break,
            };

            next = &mut next.as_mut().unwrap().next;
        }

        return head;
    }
}
