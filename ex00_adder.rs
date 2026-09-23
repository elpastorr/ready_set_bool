pub fn adder(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let ret = a & b;
        a = a ^ b;
        b = ret << 1;
    }
    a
}

pub fn test_adder(a: u32, b: u32) {
    println!("{a} + {b} = {}", adder(a, b));
}
