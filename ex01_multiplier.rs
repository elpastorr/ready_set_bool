use ex00_adder::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
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