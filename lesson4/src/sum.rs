pub fn sum(items:&[u32]) -> Option<u32>{
    let mut sum_num:u32 = 0;
    for item in items{
        if sum_num > 2^32 -1 {
            return None;
        }
        sum_num = sum_num + item;
    }
    Some(sum_num)
}