mod ex00_adder;
mod ex01_multiplier;
mod ex02_gray_code;
mod ex03_boolean_eval;
mod ex04_truth_table;
mod ex05_negation_normal_form;
mod ex06_conjunctive_normal_form;
mod ex07_sat;
mod ex08_powerset;
mod ex09_eval_set;

fn main() {
    let mut _a: u32 = 6;
    let _b: u32 = 7;
    let vec: Vec<i32> = Vec::new();
    vec.push(21);
    vec.push(42);
    vec.push(69);
    let sets = vec![
        vec![0, 1, 3, 5],
        vec![0, 3, 2, 4],
        vec![3, 42, 21],
    ];

    println!("ex00:");
    ex00_adder::test_adder(_a, _b);

    println!("\nex01:");
    ex01_multiplier::test_multiplier(_a, _b);

    println!("\nex02:");
    ex02_gray_code::test_gray_code(_a);
    ex02_gray_code::test_gray_code(_b);

    println!("\nex03:");
    ex03_boolean_eval::test_eval_formula("1011||=");

    println!("\nex04:");
    ex04_truth_table::test_print_truth_table("AB&A!B!&|");
    ex04_truth_table::test_print_truth_table("A!B|B!A|&");

    println!("\nex05:");
    ex05_negation_normal_form::test_negation_normal_form("AB&!");
    ex05_negation_normal_form::test_negation_normal_form("AB|!");
    ex05_negation_normal_form::test_negation_normal_form("AB>");
    ex05_negation_normal_form::test_negation_normal_form("AB=");
    ex05_negation_normal_form::test_negation_normal_form("AB|C&!");

    println!("\nex06:");
    ex06_conjunctive_normal_form::test_conjunctive_normal_form("AB|C|D|");
    ex06_conjunctive_normal_form::test_conjunctive_normal_form("AB&C&D&");
    ex06_conjunctive_normal_form::test_conjunctive_normal_form("AB&!C!|");

    println!("\nex07:");
    ex07_sat::test_sat("AB&");
    ex07_sat::test_sat("AB|");
    ex07_sat::test_sat("AA!&");
    ex07_sat::test_sat("AA^");

    println!("\nex08:");
    ex08_powerset::test_powerset(vec);

    println!("\nex09:");
    ex09_eval_set::test_eval_set("AB&C|", sets);
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