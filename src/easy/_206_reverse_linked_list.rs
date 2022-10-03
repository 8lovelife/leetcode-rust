use crate::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pre = None;
    let mut curr = head;
    while let Some(mut node) = curr  {
        let next = node.next;
        node.next = pre;
        pre = Some(node);
        curr = next;
    }
    return pre;
}

#[cfg(test)]
mod tests{
    use crate::{ListNode, easy::_206_reverse_linked_list::reverse_list};


    #[test]
    fn test_reverse_list(){
        let l1_val = [9,10,1]; 
        let mut dummy_node = Some(Box::new(ListNode::new(0)));
        let mut curr = dummy_node.as_mut();
        for &value in l1_val.iter() {
            if let Some(mut node) = curr {                
                let next = Box::new(ListNode::new(value));
                node.next = Some(next);
                curr = node.next.as_mut();
            }
        }
        let reverse_node =  reverse_list(dummy_node.unwrap().next);
        println!("{:#?}",reverse_node);

    }
}