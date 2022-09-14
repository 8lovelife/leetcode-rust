mod easy;

#[cfg(test)]
mod tests {

    use super::easy::two_sum_efficiency;

    #[test]
    fn test_two_sum_efficiency() {
        two_sum_efficiency(([3,2,8,10]).to_vec(),10);
        
    }
}
