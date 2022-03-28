fn moveHanoi(src: &str, dst: &str, n: u32) {
    println!("move plate {} from {} to {} \n", n, src, dst);
}

pub fn hanoiCalc(src: &str, dst: &str, aux: &str, n: u32) {
    if n == 1 {
        moveHanoi(src, dst, 1);
        return;
    }

    hanoiCalc(src, aux, dst, n - 1);
    moveHanoi(src, dst, n);
    hanoiCalc(aux, dst, src, n - 1);
}
