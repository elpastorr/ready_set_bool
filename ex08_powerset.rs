fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
	let mut powerset: Vec<Vec<i32>> = Vec::new();
	let nb_comb = 2_i32.pow(set.len() as u32);

	for i in 0..nb_comb {
		let mut subset: Vec<i32> = Vec::new();
		let mut bit : i32 = 1;

		for s in &set {
			if i & bit != 0 {
				subset.push(*s);
			}
			bit = bit << 1;
		}
		powerset.push(subset);
	}
	return powerset;
}

pub fn test_powerset(set: Vec<i32>) {
	println!("Powerset of {:?}:", set);
	println!("{:?}", powerset(set));
}