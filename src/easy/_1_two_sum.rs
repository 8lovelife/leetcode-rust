use std::collections::HashMap;

pub fn two_sum_efficiency(nums:Vec<i32>,target:i32)->Vec<i32>{
    let mut container = HashMap::with_capacity(nums.len());
    for (i,x) in nums.iter().enumerate(){
        let search = target - x;
        if let Some(&match_index) = container.get(&search){
           return vec![match_index as i32 , i as i32];
        } else {
            container.insert(x, i);
        }
    }
    panic!("no matched value");
}

pub fn two_sum_normal(nums:Vec<i32>,target:i32)->Vec<i32> {
    for i in 0..nums.len() {
        let search = target-nums[i];
        for j in (i+1)..nums.len() {
            if nums[j] == search {
               return vec![i as i32,j as i32]; 
            }
        }
    }
    panic!("no matched value")
}


#[cfg(test)]
mod tests {

    use super::{two_sum_efficiency,two_sum_normal};

    #[test]
    fn test_two_sum_efficiency() {
        let target = 7;
       let pos = two_sum_efficiency([3,2,4,8,20].to_vec(),target);
       println!("target {} found! position in {:#?}",target,pos);
    }

    #[test]
    fn test_two_sum_normal(){
        let target = 7;
        let pos = two_sum_normal([3,2,4,8,20].to_vec(),target);
        println!("target {} found! position in {:#?}",target,pos); 
    }
}