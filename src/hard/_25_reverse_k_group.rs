use std::collections::VecDeque;

use crate::ListNode;

pub fn _25_reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut header = head;
    let mut count = 0;
    let k = k as usize;
    let mut queue = VecDeque::with_capacity(k);
    while let Some(mut node) = header {
        header = node.next.take();
        queue.push_back(Some(node));
        count += 1;
        if count == k {
            break;
        }
    }
    if queue.len() == k {
        let mut pre = _25_reverse_k_group(header, k as i32);
        while let Some(link) = queue.pop_front() {
            if let Some(mut node) = link {
                node.next = pre;
                pre = Some(node);
            }
        }
        pre
    } else {
        let mut pre = None;
        while let Some(link) = queue.pop_back() {
            if let Some(mut node) = link{
                node.next = pre;
                pre = Some(node);
            }
        }
        pre
    }
    // return dummy_node.as_ref().unwrap().next
}

#[cfg(test)]
mod tests{
    use crate::{ListNode, hard::_25_reverse_k_group::_25_reverse_k_group};


    #[test]
    fn test_reverse_list(){
        let l1_val = [9,10,1,5,2]; 
        let mut dummy_node = Some(Box::new(ListNode::new(0)));
        let mut curr = dummy_node.as_mut();
        for &value in l1_val.iter() {
            if let Some(mut node) = curr {                
                let next = Box::new(ListNode::new(value));
                node.next = Some(next);
                curr = node.next.as_mut();
            }
        }
        let reverse_node =  _25_reverse_k_group(dummy_node.unwrap().next,3);
        println!("{:#?}",reverse_node);

    }
}
