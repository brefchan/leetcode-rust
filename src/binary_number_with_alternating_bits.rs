pub fn has_alternating_bit(n: i32) -> bool {
    let a = n ^ (n >> 1);
    return a & (a + 1) == 0;
}
