// mod ex00_adder;
// mod ex01_multiplier;
// mod ex02_gray_code;
mod ex03_boolean_eval;
mod ex04_truth_table;
// mod ex05_negation_normal_form;
// mod ex06_conjunctive_normal_form;

fn main() {
    // let mut _a: u32 = 6;
    // let _b: u32 = 7;

    // println!("ex00:");
    // println!("{_a} + {_b} = {}", ex00_adder::adder(_a, _b));

    // println!("\nex01:");
    // println!("{_a} * {_b} = {}", ex01_multiplier::multiplier(_a, _b));

    // println!("\nex02:");
    // println!("gray code of {_a} = {}", ex02_gray_code::gray_code(_a));
    // println!("gray code of {_b} = {}", ex02_gray_code::gray_code(_b));

    // println!("\nex03:");
    // let oui: &str = "1011||=";
    // println!("{oui} -> {}", ex03_boolean_eval::eval_formula(oui));

    // println!("\nex04:");
    // ex04_truth_table::print_truth_table("AB&A!B!&|");
    // println!("\n");
    // ex04_truth_table::print_truth_table("A!B|B!A|&");



    // println!("\nex05:");
    // println!("{}", ex05_negation_normal_form::negation_normal_form("AB&!"));
    // println!("{}", ex05_negation_normal_form::negation_normal_form("AB|!"));
    // println!("{}", ex05_negation_normal_form::negation_normal_form("AB>"));
    // println!("{}", ex05_negation_normal_form::negation_normal_form("AB="));
    // println!("{}", ex05_negation_normal_form::negation_normal_form("AB|C&!"));
}

// fn main() {
//     // create some strings
//     let str1 = "Educative is the best platform!";
//     let str2 = "Rust";
//     let str3 = "Welcome to Edpresso";
//     let str4 = "Programming";
  
//     // create the matches
//     let match1 = "is";
//     let match2 = 'R';
//     let match3 = "to";
//     let match4 = "23";
  
//     // find the matches and print byte indices
//     println!(" {:?}", str1.find(match1));
//     println!(" {:?}", str2.find(match2));
//     println!(" {:?}", str3.find(match3));
//     println!(" {:?}", str4.find(match4));
//     if str1.find(match1).is_some() {
//         println!("1");
//     }
//     if str2.find(match2).is_some() {
//         println!("2");
//     }
//     if str3.find(match3).is_some() {
//         println!("3");
//     }
//     if str4.find(match4).is_some() {
//         println!("4");
//     }
// }