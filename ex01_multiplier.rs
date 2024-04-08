use ex00_adder::adder;

fn multiplier(a: u32, b: u32) -> u32 {
    if b == 0 {
        return 0;
    }
    if b & 1 == 0 {
        multiplier(a, b >> 1) << 1
    }
    else {
        adder(a, multiplier(a, b >> 1) << 1)
    }
}

pub fn test_multiplier(a: u32, b: u32) {
    println!("{a} * {b} = {}", multiplier(a, b));
}