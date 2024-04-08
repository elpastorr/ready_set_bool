fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

pub fn test_gray_code(n: u32){
    println!("gray code of {n} = {}", gray_code(n));
}