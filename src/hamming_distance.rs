pub fn hamming_distance(x: i32, y: i32) -> i32 {
    let mut z = x ^ y;
    let mut count = 0;
    while (z > 0) {
        count += (z & 1);
        z = z >> 1;
    }
    count
}

pub fn kernighan_hamming_distance(x: i32, y: i32) -> i32 {
    let mut z = x ^ y;
    let mut count = 0;
    while z > 0 {
        z = z & (z - 1);
        count += 1;
    }
    count
}
