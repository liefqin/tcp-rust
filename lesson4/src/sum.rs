pub fn sum(items: &[u32]) -> Option<u32> {
    let mut sum_num: u32 = 0;
    for item in items {
        match sum_num.checked_add(*item) {
            Some(sum) => {
                sum_num = sum;
            }
            None => { return None; }
        }
    }
    Some(sum_num)
}