pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy_head = Some(Box::new(ListNode::new(0)));
    let mut curr = dummy_head.as_mut();
    let mut carry = 0;
    let mut l1_list = l1;
    let mut l2_list = l2;
    while l1_list.is_some() || l2_list.is_some(){
        let mut sum = carry;
        if let Some(node) = l1_list {
            sum += node.val;
            l1_list = node.next;
        }
        if let Some(node) = l2_list {
            sum += node.val;
            l2_list = node.next;
        }
        carry = sum / 10;
        let remainder = sum % 10;
        if let Some(node) = curr {
            node.next = Some(Box::new(ListNode {
                val: remainder,
                next: None,
            }));
            curr = node.next.as_mut();
        }
    }
    if carry!=0 {
       curr.unwrap().next = Some(Box::new(ListNode::new(carry)));
    }
    return dummy_head.unwrap().next;
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    fn new(val:i32) -> Self{
        ListNode {
            val,
            next:None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::add_two_numbers;
    use super::ListNode;

    #[test]
    fn test_add_two_numbers() {
        let l1_val = [9,9];
        let l2_val = [1];
        let mut l1_dummy_head = Some(Box::new(ListNode::new(0)));
        let mut curr_l1 = l1_dummy_head.as_mut();
        for &i in l1_val.iter() {
            if let Some(node) = curr_l1 {
                node.next = Some(Box::new(ListNode::new(i)));
                curr_l1 = node.next.as_mut();
            }
        }
        let mut l2_dummy_head = Some(Box::new(ListNode::new(0)));
        let mut curr_l2 = l2_dummy_head.as_mut();
        for &j in l2_val.iter() {
            if let Some(node) = curr_l2 {
                node.next = Some(Box::new(ListNode::new(j)));
                curr_l2 = node.next.as_mut();
            }
        }
        let l1 =  l1_dummy_head.unwrap().next;
        let l2 =  l2_dummy_head.unwrap().next;
        println!(
            "l1_node is {:#?},l2_node is {:#?}",
            l1,
            l2,
        );
       let l3 =  add_two_numbers(l1, l2);
       println!("l3_node is {:#?}",l3);
    }
}
