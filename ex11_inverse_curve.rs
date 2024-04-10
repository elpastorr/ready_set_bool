fn reverse_map(n: f64) -> (u16, u16) {
	let max_u32: f64 = u32::MAX.into();
	let tmp: f64 = n * max_u32;
	let res: u32 = tmp as u32;
	
	let mut x: u16 = 0;
	let mut y: u16 = 0;
	let mut tbit: u32 = 1;
	let mut sbit: u16 = 1;

	for _i in 0..16 {
		if tbit & res != 0 {
			x = x | sbit;
		}
		tbit = tbit << 1;
		if tbit & res != 0 {
			y = y | sbit;
		}
		tbit = tbit << 1;
		sbit = sbit << 1;
	}
	return (x, y);
}

pub fn test_reverse_map(n: f64) {
	println!("converting {n}: {:?}", reverse_map(n));
}