pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let _k = k as usize;
    let mut max = 0;

    if (_k >= nums.len()) {
        max = nums.iter().sum();
    } else {
        let mut sum = 0;
        sum = nums[0.._k].iter().sum();
        max = sum;
        println!("k = {},max = {}", k, max);
        for i in _k..nums.len() {
            sum += nums[i];
            sum -= nums[i - _k];
            max = max.max(sum);
            println!("max = {}, sum = {}", max, sum);
        }
    }

    (max as f64 / k as f64)
}
