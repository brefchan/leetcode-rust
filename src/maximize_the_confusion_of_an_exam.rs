pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
    let bytes = answer_key.as_bytes();
    getCnt(b'T', &bytes, k).max(getCnt(b'F', &bytes, k))
}

fn getCnt(c: u8, bytes: &[u8], k: i32) -> i32 {
    let mut ans = 0;
    for i in 0..bytes.len() {
        let mut j = 0;
        let mut cnt = 0;
        if bytes[i] == c {
            cnt += 1;
        }

        while cnt > k {
            if bytes[j] == c {
                cnt -= 1;
            }
            j += 1;
        }

        ans = ans.max(i - j + 1);
    }
    ans as i32
}
