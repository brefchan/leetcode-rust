pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    let mut result = Vec::<i32>::new();
    (left..=right).for_each(|i| {
        if i % 10 == 0 {
            return;
        }
        if i < 10 {
            result.push(i);
            return;
        }

        let mut _index = 1;
        loop {
            let div_number = i % 10_i32.pow(_index) / 10_i32.pow(_index - 1);
            println!("mod_number:{}", div_number);

            if div_number == 0 {
                return;
            }
            if div_number == 0 {
                result.push(i);
                return;
            }
            if i % div_number != 0 {
                return;
            }
            _index += 1;
        }
    });

    result
}

pub fn self_dividing_numbers2(left: i32, right: i32) -> Vec<i32> {
    let mut result = Vec::<i32>::new();
    (left..=right).for_each(|i| {
        let mut cur = i;
        while i != 0 {
            let div_num = cur % 10;
            if i % div_num != 0 && div_num == 0 {
                return;
            }
            cur = cur / 10;
        }
        result.push(i);
    });
    result
}
