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

pub fn print_truth_table(formula: &str) {
    let mut letters: Vec<char> = Vec::new();
    for c in formula.chars() {
        if !valid_input(c) {
            println!("Invalid char: '{}'", c as char);
            return;
        }
        if c.is_alphabetic() && !letters.contains(&c) {
            letters.push(c);
        }
    }
    for i in letters.iter() {
        print!("| {} ", *i as char);
    }
    println!( "| = |");
    for _ in letters.iter() {
        print!("|---");
    }
    println!("|---|");

    let nb_comb = 2_u32.pow(letters.len() as u32);
    for i in 0..nb_comb {
        let mut binar_i = format!("{:b}", i);
        while binar_i.len() <  letters.len() {
            binar_i = format!("0{}", binar_i);
        }
        for c in binar_i.chars() {
            print!("| {} ", c);
        }
        let mut formula = formula.to_string();
        for (i, c) in letters.iter().enumerate() {
            formula = formula.replace(*c, &binar_i[i..i+1]);
        }

        println!("| {} |", eval_formula(&formula) as u32);
    }
    // println!("{nb_comb}");
    // println!("len = {}", 2_u32.pow(letters.len() as u32));
}
