#[cfg(test)]
use super::*;

#[test]
fn test_avbv() {
    let bv = "BV1r54y1p7ah";
    let av = utils::bv2av(bv);
    println!("AV: {}", av);
    let bv_new = utils::av2bv(av);
    println!("BV: {}", bv_new);
}
