pub fn map(x: u16, y: u16) -> f64 {
	let mut res: u32 = 0;
	let mut tbit: u32 = 1;
	let mut sbit: u16 = 1;

	for _i in 0..16 {
		if sbit & x != 0 {
			res = res | tbit;
		}
		tbit = tbit << 1;
		if sbit & y != 0 {
			res = res | tbit;
		}
		tbit = tbit << 1;
		sbit = sbit << 1;
	}

	let result: f64 = res.into();
	let max_u32: f64 = u32::MAX.into();

	return result / max_u32;
}

pub fn test_map(x: u16, y: u16) {
	println!("converting {x} & {y}: {}", map(x, y));
}