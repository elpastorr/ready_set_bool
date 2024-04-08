use ex03_boolean_eval::eval_formula;

fn valid_input(c: char) -> bool {
    if (c >= 'A') && (c <= 'Z') {
        return true;
    }
    if (c == '!') || (c == '&') || (c == '|') || (c == '^') || (c == '>') || (c == '=') {
        return true;
    }
    return false;
}

fn sat(formula: &str) -> bool {
	let mut letters: Vec<char> = Vec::new();
    for c in formula.chars() {
        if !valid_input(c) {
            println!("Invalid char: '{}'", c as char);
            return false;
        }
        if c.is_alphabetic() && !letters.contains(&c) {
            letters.push(c);
        }
    }
    let nb_comb = 2_u32.pow(letters.len() as u32);
    for i in 0..nb_comb {
        let mut binar_i = format!("{:b}", i);
        while binar_i.len() <  letters.len() {
            binar_i = format!("0{}", binar_i);
        }
        let mut formula = formula.to_string();
        for (i, c) in letters.iter().enumerate() {
            formula = formula.replace(*c, &binar_i[i..i+1]);
        }
		if eval_formula(&formula) == true {
			return true;
		}
    }
	return false;
}

pub fn test_sat(formula: &str) {
	println!("{formula} -> {}", sat(formula));
}