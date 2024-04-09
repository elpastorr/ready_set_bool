use std::str::Chars;

struct OpNode {
    value: char,
    left: Option<Box<OpNode>>,
    right: Option<Box<OpNode>>,
}

impl OpNode {
    fn new() -> OpNode {
        OpNode {
            value: '*',
            left: None,
            right: None
        }
    }

    fn copy(&mut self) -> Option<Box<OpNode>> {
        let mut new_node = OpNode::new();
        new_node.value = self.value;

        match self.left.as_mut() {
            None => (),
            Some(left) => new_node.left = left.copy(),
        }
        match self.right.as_mut() {
            None => (),
            Some(right) => new_node.right = right.copy(),
        }
        return Some(Box::new(new_node));
    }

    fn fill_node(&mut self, ops: &mut Chars) {
        let op = ops.next_back();
        if op.is_none() { return ; }
        self.value = op.unwrap();
    
        if "!&|^>=".contains(op.unwrap()) {
            let new_node = Some(Box::new(OpNode::new()));
            self.right = new_node;
            self.right.as_mut().unwrap().fill_node(ops);

            if "&|^>=".contains(op.unwrap()) {
                let new_node = Some(Box::new(OpNode::new()));
                self.left = new_node;
                self.left.as_mut().unwrap().fill_node(ops);
            }
        }
    }

    fn to_string(&mut self) -> String {
        let mut out = String::from("");

        match self.left.as_mut() {
            Some(left) => out += &left.to_string(),
            None => (),
        }

        match self.right.as_mut() {
            Some(right) => out += &right.to_string(),
            None => (),
        }
        if self.value == '*' {
            println!("Invalid formula ! The result is not valid");
        }
        out.push(self.value);
        return out;
    }

    fn change_form(&mut self) {
        match self.left.as_mut() {
            Some(left) => left.change_form(),
            None => (),
        }

        match self.right.as_mut() {
            Some(right) => right.change_form(),
            None => (),
        }

        if self.value == '!' {
            if self.right.as_ref().unwrap().value == '!' {
                self.value = self.right.as_ref().unwrap().right.as_ref().unwrap().value;
                self.left = self.right.as_mut().unwrap().right.as_mut().unwrap().left.take();
                self.right = self.right.as_mut().unwrap().right.as_mut().unwrap().right.take();
                self.change_form();
            }
            else if "&|".contains(self.right.as_ref().unwrap().value) {
                if self.right.as_ref().unwrap().value == '&' {
                    self.value = '|';
                }
                else {
                    self.value = '&';
                }

                let mut left_node = OpNode::new();
                left_node.value = '!';
                left_node.right = self.right.as_mut().unwrap().left.take();

                let mut right_node = OpNode::new();
                right_node.value = '!';
                right_node.right = self.right.as_mut().unwrap().right.take();
                
                self.left = Some(Box::new(left_node));
                self.right = Some(Box::new(right_node));
                self.change_form();
            }
        }
        else if self.value == '^' {
            self.value = '|';

            let mut right_left_node = OpNode::new();
            right_left_node.value = '!';
            right_left_node.right = self.left.as_mut().unwrap().copy();

            let mut left_right_node = OpNode::new();
            left_right_node.value = '!';
            left_right_node.right = self.right.as_mut().unwrap().copy();

            let mut right_node = OpNode::new();
            right_node.value = '&';
            right_node.right = self.right.as_mut().unwrap().copy();
            right_node.left = Some(Box::new(right_left_node));

            let mut left_node = OpNode::new();
            left_node.value = '&';
            left_node.right = Some(Box::new(left_right_node));
            left_node.left = self.left.as_mut().unwrap().copy();

            self.left = Some(Box::new(left_node));
            self.right = Some(Box::new(right_node));
            self.change_form();
        }
        else if self.value == '=' {
            self.value = '&';

            let mut right_node = OpNode::new();
            right_node.value = '>';
            right_node.right = self.right.as_mut().unwrap().copy();
            right_node.left = self.left.as_mut().unwrap().copy();

            let mut left_node = OpNode::new();
            left_node.value = '>';
            left_node.right = self.left.as_mut().unwrap().copy();
            left_node.left = self.right.as_mut().unwrap().copy();

            self.left = Some(Box::new(left_node));
            self.right = Some(Box::new(right_node));
            self.change_form();
        }
        else if self.value == '>' {
            self.value = '|';

            let mut left_node = OpNode::new();
            left_node.value = '!';
            left_node.right = self.left.as_mut().unwrap().copy();

            self.left = Some(Box::new(left_node));
            self.change_form();
        }
	}

	fn eval_sets(&mut self, sets: Vec<Vec<i32>>) -> Vec<i32> {
		let formula = self.to_string();
		let mut vec: Vec<Vec<i32>> = Vec::new();
		let mut vars: String = String::from("");
		let mut all_sets: Vec<i32> = Vec::new();

		for c in 'A'..'Z' {
			if formula.contains(c) {
				vars.push(c);
			}
		}
		if sets.len() != vars.len() {
			panic!("Error: Number of vars in formula is different from number of sets"); 
		}

		for set in &sets {
			for n in set {
				if !all_sets.contains(&n) {
					all_sets.push(*n);
				}
			}
		}

		for c in formula.chars() {
			if c.is_ascii_uppercase() {
				vec.push(sets[vars.find(c).unwrap()].clone());
			}
			else if c == '!' {
				if vec.len() < 1 {
					panic!("Error: Invalid input");
				}
				let right: Vec<i32> = vec.pop().unwrap();
				let mut inverse: Vec<i32> = Vec::new();
				for n in &all_sets {
					if !right.contains(&n) {
						inverse.push(*n);
					}
				}
				vec.push(inverse);
			}
			else if "&|".contains(c) {
				if vec.len() < 2 {
					panic!("Error: Invalid input");
				}
				let right: Vec<i32> = vec.pop().unwrap();
				let left: Vec<i32> = vec.pop().unwrap();
				
				if c == '|' {
					let mut or: Vec<i32> = Vec::new();
					for n in &right {
						or.push(*n);
					}
					for n in &left {
						if !or.contains(&n) {
							or.push(*n);
						}
					}
					vec.push(or);
				}
				else {
					let mut and: Vec<i32> = Vec::new();
					for n in &right {
						if left.contains(&n) {
							and.push(*n);
						}
					}
					vec.push(and);
				}
			}
		}
		if vec.len() != 1 {
			panic!("Error: Invalid input");
		}
		return vec.pop().unwrap();
	}
}


fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
	let mut to_char = formula.chars();
    let mut node = OpNode::new();
    node.fill_node(&mut to_char);
    node.change_form();

    return node.eval_sets(sets);
}

pub fn test_eval_set(formula: &str, sets: Vec<Vec<i32>>) {
	println!("formula: {formula}\nsets: {:?}", sets);
	println!("result: {:?}", eval_set(formula, sets));
}