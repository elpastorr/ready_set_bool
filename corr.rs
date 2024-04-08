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

    fn parse_op(&mut self, ops: &mut Chars) {
        let op = ops.next_back();
        if op.is_none() {
            return ;
        }

        self.value = op.unwrap();

        if "!&|^>=".contains(op.unwrap()) {
            let new_node = Some(Box::new(OpNode::new()));
            self.right = new_node;
            self.right.as_mut().unwrap().parse_op(ops);

            if "&|^>=".contains(op.unwrap()) {
                let new_node = Some(Box::new(OpNode::new()));
                self.left = new_node;
                self.left.as_mut().unwrap().parse_op(ops);
            }
        }
    }

    fn stringify(&mut self) -> String {
        let mut expr = String::from("");

        match self.left.as_mut() {
            None => (),
            Some(left) => expr += &left.stringify(),
        }

        match self.right.as_mut() {
            None => (),
            Some(right) => expr += &right.stringify(),
        }

        expr.push(self.value);

        return expr;
    }

    fn negify(&mut self) {
        match self.left.as_mut() {
            None => (),
            Some(left) => left.negify(),
        }

        match self.right.as_mut() {
            None => (),
            Some(right) => right.negify(),
        }

        if self.value == '!' {
            if self.right.as_ref().unwrap().value == '!' {
                self.value = self.right.as_ref().unwrap().right.as_ref().unwrap().value;
                self.left = self.right.as_mut().unwrap().right.as_mut().unwrap().left.take();
                self.right = self.right.as_mut().unwrap().right.as_mut().unwrap().right.take();
                self.negify();
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
                self.negify();
            }
        }
        else if self.value == '^' {
            self.value = '|';
            
            let mut left_negation_node = OpNode::new();
            left_negation_node.value = '!';
            left_negation_node.right = self.left.as_mut().unwrap().copy();

            let mut right_negation_node = OpNode::new();
            right_negation_node.value = '!';
            right_negation_node.right = self.right.as_mut().unwrap().copy();

            let mut left_node = OpNode::new();
            left_node.value = '&';
            left_node.left = self.left.as_mut().unwrap().copy();
            left_node.right = self.right.as_mut().unwrap().copy();

            let mut right_node = OpNode::new();
            right_node.value = '&';
            right_node.left = Some(Box::new(left_negation_node));
            right_node.right = Some(Box::new(right_negation_node));

            self.left = Some(Box::new(left_node));
            self.right = Some(Box::new(right_node));
            self.negify();
        }
        else if self.value == '>' {
            self.value = '|';

            let mut left_node = OpNode::new();
            left_node.value = '!';
            left_node.right = self.left.as_mut().unwrap().copy();

            self.left = Some(Box::new(left_node));
            self.negify();
        }
        else if self.value == '=' {
            self.value = '&';
            
            let mut left_node = OpNode::new();
            left_node.value = '>';
            left_node.left = self.left.as_mut().unwrap().copy();
            left_node.right = self.right.as_mut().unwrap().copy();

            let mut right_node = OpNode::new();
            right_node.value = '>';
            right_node.left = self.right.as_mut().unwrap().copy();
            right_node.right = self.left.as_mut().unwrap().copy();

            self.left = Some(Box::new(left_node));
            self.right = Some(Box::new(right_node));
            self.negify();
        }
    }

    fn possible_conjuction(&mut self) -> bool {
        if "&|".contains(self.value) {
            if self.left.as_ref().unwrap().value == self.value {
                if self.right.as_ref().unwrap().value.is_ascii_uppercase() {
                    return true;
                }
                else if self.right.as_ref().unwrap().value == '!' && self.right.as_ref().unwrap().right.as_ref().unwrap().value.is_ascii_uppercase() {
                    return true;
                }
            }
        }
        return false;
    }

    fn conjuctify(&mut self) {
        match self.left.as_mut() {
            None => (),
            Some(left) => left.conjuctify(),
        }

        match self.right.as_mut() {
            None => (),
            Some(right) => right.conjuctify(),
        }

        if self.possible_conjuction() {
            std::mem::swap(&mut self.left, &mut self.right);
            self.conjuctify();
        }
    }
}

fn conjunctive_normal_form(formula: &str) -> String {
    let mut chars = formula.chars();
    let mut root = OpNode::new();

    root.parse_op(&mut chars);

    root.negify();
    root.conjuctify();

    return root.stringify();
}

fn main() {
    println!("{}", conjunctive_normal_form("AB&!"));
    // A!B!|
    println!("{}", conjunctive_normal_form("AB|!"));
    // A!B!&
    println!("{}", conjunctive_normal_form("AB|C&"));
    // AB|C&
    println!("{}", conjunctive_normal_form("AB|C|D|"));
    // ABCD|||
    println!("{}", conjunctive_normal_form("AB&C&D&"));
    // ABCD&&&
    println!("{}", conjunctive_normal_form("AB&!C!|"));
    // A!B!C!||
    println!("{}", conjunctive_normal_form("AB|!C!&"));
    // A!B!C!&&
}