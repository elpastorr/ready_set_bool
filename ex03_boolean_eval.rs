fn valid_char(c: u8) -> bool {
    if (c == b'0') || (c == b'1') || (c == b'!') || (c == b'&') || (c == b'|') || (c == b'^') || (c == b'>') || (c == b'=') {
        return true;
    }
    return false;
}

fn calculate(a: bool, b: bool, op: u8) -> bool {
    match op {
        b'&' => a && b,
        b'|' => a || b,
        b'^' => a ^ b,
        b'>' => !a || b,
        b'=' => a == b,
        _=>  unreachable!("Error"),
    }
}

pub fn eval_formula(formula: &str) -> bool {
    let mut vec = Vec::new();
    for &c in formula.as_bytes() {
        if !valid_char(c) {
            println!("Invalid char: '{}'", c as char);
            return false;
        }
        match c {
            b'1' | b'0' => vec.push((c - b'0') != 0),
            b'!' if vec.len() > 0 => {
                let tmp = vec.pop();
                vec.push(!tmp.expect(""));
            }
            b'&' | b'|' | b'^' | b'>'| b'=' if vec.len() > 1 => {
                let b = vec.pop().expect("");
                let a = vec.pop().expect("");
                vec.push(calculate(a, b, c));
            }
            _=> println!("Missing value for operator '{}'", c as char),
        };
    }
    return vec.pop().expect("");
}