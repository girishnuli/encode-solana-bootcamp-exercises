use std::collections::HashMap;

fn two_sum_loop(nums: Vec<i32>, target: i32) -> Vec<i32> {    
    let total_nums = nums.len();
    let mut matching_num_indices = Vec::new();

    for i in 0..total_nums {
        for j in i + 1..total_nums { 
            if nums[i] + nums[j] == target {
                matching_num_indices.push(i as i32);
                matching_num_indices.push(j as i32);
                return matching_num_indices; 
            }
        }
    }
    
    matching_num_indices
}

fn two_sum_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {  
    let mut matching_num_indices = Vec::new();
    let mut checked_numbers: HashMap<i32, usize> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&index) = checked_numbers.get(&complement) {
            matching_num_indices.push(index as i32);
            matching_num_indices.push(i as i32);
            return matching_num_indices; 
        }

        checked_numbers.insert(num, i);
    }

    matching_num_indices

}

fn main() {  
    println!("Loop solution: {:?}", two_sum_loop(vec![2, 3, 4, 5,], 9));  
    println!("Hashmap solution: {:?}", two_sum_hashmap(vec![2, 3, 4, 5,], 9));  
}
