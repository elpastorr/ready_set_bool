use ex00_adder::adder;

fn multiplier(mut a: u32, mut b: u32) -> u32 {
    let mut result = 0;

    while b != 0 {
        if b & 1 != 0 {
            result = adder(result, a)
        }
        a = a << 1;
        b = b >> 1;
        println!("a = {a}, b = {b}");
    }

    result
}

pub fn test_multiplier(a: u32, b: u32) {
    println!("{a} * {b} = {}", multiplier(a, b));
}
