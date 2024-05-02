use std::collections::HashMap;

const TABLE: &str = "fZodR9XQDSUm21yCkr6zBqiveYah8bt4xsWpHnJE7jL5VG3guMTKNPAwcF";
const XOR: i64 = 177451812;
const ADD: i64 = 8728348608;
const S: [usize; 6] = [11, 10, 3, 8, 4, 6];

pub(crate) fn bv2av(bv: &str) -> i64 {
    let tr: HashMap<char, i64> = TABLE.chars().enumerate().map(|(i, c)| (c, i as i64)).collect();
    let mut r = 0;

    for i in 0..6 {
        let index = tr[&bv.chars().nth(S[i]).unwrap()];
        r += index * 58_i64.pow(i as u32);
    }

    (r - ADD) ^ XOR
}

pub(crate) fn av2bv(av: i64) -> String {
    let x = (av ^ XOR) + ADD;
    let mut result = String::with_capacity(12);

    let table_bytes = TABLE.as_bytes();
    let mut r = ['B', 'V', '1', ' ', ' ', '4', ' ', '1', ' ', '7', ' ', ' '];

    for i in 0..6 {
        let index = ((x / 58_i64.pow(i as u32)) % 58) as usize;
        r[S[i]] = table_bytes[index] as char;
    }

    result.extend(&r);
    result
}