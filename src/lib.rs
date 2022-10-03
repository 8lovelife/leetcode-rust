mod easy;
mod medium;
mod hard;


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

    use super::easy::two_sum_efficiency;

    #[test]
    fn test_two_sum_efficiency() {
        two_sum_efficiency(([3,2,8,10]).to_vec(),10);
        
    }
}
