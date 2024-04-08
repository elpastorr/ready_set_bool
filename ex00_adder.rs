pub fn adder(a: u32, b: u32) -> u32 {
    let res = a ^ b;
    let ret = (a & b) << 1;
    if ret == 0 {
        res
    } else {
        adder(res, ret)
    }
}

pub fn test_adder(a: u32, b: u32) {
    println!("{a} + {b} = {}", adder(a, b));
}