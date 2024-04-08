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
        else if self.value == '^' {// A^B -> (A&!B)|(!A&B)
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
        else if self.value == '=' {// A=B -> (A>B)&(B>A) ??? AB&A!B!&|
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
        else if self.value == '>' {// A>B -> !A|B
            self.value = '|';

            let mut left_node = OpNode::new();
            left_node.value = '!';
            left_node.right = self.left.as_mut().unwrap().copy();

            self.left = Some(Box::new(left_node));
            self.change_form();
        }
    }
}


pub fn negation_normal_form(formula: &str) -> String {
    let mut to_char = formula.chars();
    let mut node = OpNode::new();
    node.fill_node(&mut to_char);
    node.change_form();

    return node.to_string();
}