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
mod ex10_curve;
use ex10_curve::map;
mod ex11_inverse_curve;

fn main() {
    println!("ex00:");
    ex00_adder::test_adder(0, 0);
    ex00_adder::test_adder(1, 0);
    ex00_adder::test_adder(0, 1);
    ex00_adder::test_adder(1, 1);
    ex00_adder::test_adder(1, 2);
    ex00_adder::test_adder(2, 2);


    println!("\nex01:");
    ex01_multiplier::test_multiplier(0, 0);
    ex01_multiplier::test_multiplier(1, 0);
    ex01_multiplier::test_multiplier(0, 1);
    ex01_multiplier::test_multiplier(1, 1);
    ex01_multiplier::test_multiplier(1, 2);
    ex01_multiplier::test_multiplier(2, 2);


    println!("\nex02:");
    for i in 0..16 {
        ex02_gray_code::test_gray_code(i);
    }


    println!("\nex03:");
    ex03_boolean_eval::test_eval_formula("0!");
    ex03_boolean_eval::test_eval_formula("1!");
    ex03_boolean_eval::test_eval_formula("00|");
    ex03_boolean_eval::test_eval_formula("10|");
    ex03_boolean_eval::test_eval_formula("01|");
    ex03_boolean_eval::test_eval_formula("11|");
    ex03_boolean_eval::test_eval_formula("10&");
    ex03_boolean_eval::test_eval_formula("11&");
    ex03_boolean_eval::test_eval_formula("11^");
    ex03_boolean_eval::test_eval_formula("10^");
    ex03_boolean_eval::test_eval_formula("00>");
    ex03_boolean_eval::test_eval_formula("01>");
    ex03_boolean_eval::test_eval_formula("10>");
    ex03_boolean_eval::test_eval_formula("11>");
    ex03_boolean_eval::test_eval_formula("00=");
    ex03_boolean_eval::test_eval_formula("11=");
    ex03_boolean_eval::test_eval_formula("10=");
    ex03_boolean_eval::test_eval_formula("01=");

    ex03_boolean_eval::test_eval_formula("11&0|");
    ex03_boolean_eval::test_eval_formula("10&1|");
    ex03_boolean_eval::test_eval_formula("11&1|");
    ex03_boolean_eval::test_eval_formula("11&1|1^");
    ex03_boolean_eval::test_eval_formula("01&1|1=");
    ex03_boolean_eval::test_eval_formula("01&1&1&");
    ex03_boolean_eval::test_eval_formula("0111&&&");


    println!("\nex04:");
    ex04_truth_table::test_print_truth_table("A");
    ex04_truth_table::test_print_truth_table("A!");
    ex04_truth_table::test_print_truth_table("AB|");
    ex04_truth_table::test_print_truth_table("AB&");
    ex04_truth_table::test_print_truth_table("AB^");
    ex04_truth_table::test_print_truth_table("AB>");
    ex04_truth_table::test_print_truth_table("AB=");
    ex04_truth_table::test_print_truth_table("AA=");

    ex04_truth_table::test_print_truth_table("ABC==");
    ex04_truth_table::test_print_truth_table("AB>C>");
    ex04_truth_table::test_print_truth_table("AB>A>A>");


    println!("\nex05:");
    ex05_negation_normal_form::test_negation_normal_form("A");
    ex05_negation_normal_form::test_negation_normal_form("A!");
    ex05_negation_normal_form::test_negation_normal_form("AB&!");
    ex05_negation_normal_form::test_negation_normal_form("AB|!");
    ex05_negation_normal_form::test_negation_normal_form("AB>!");
    ex05_negation_normal_form::test_negation_normal_form("AB=!");

    ex05_negation_normal_form::test_negation_normal_form("ABC||");
    ex05_negation_normal_form::test_negation_normal_form("ABC||!");
    ex05_negation_normal_form::test_negation_normal_form("ABC|&");
    ex05_negation_normal_form::test_negation_normal_form("ABC&|");
    ex05_negation_normal_form::test_negation_normal_form("ABC&|!");
    ex05_negation_normal_form::test_negation_normal_form("ABC^^");
    ex05_negation_normal_form::test_negation_normal_form("ABC>>");


    println!("\nex06:");
    ex06_conjunctive_normal_form::test_conjunctive_normal_form("A");
    ex06_conjunctive_normal_form::test_conjunctive_normal_form("A!");
    ex06_conjunctive_normal_form::test_conjunctive_normal_form("AB&!");
    ex06_conjunctive_normal_form::test_conjunctive_normal_form("AB|!");
    ex06_conjunctive_normal_form::test_conjunctive_normal_form("AB>!");
    ex06_conjunctive_normal_form::test_conjunctive_normal_form("AB=!");

    ex06_conjunctive_normal_form::test_conjunctive_normal_form("ABC||");
    ex06_conjunctive_normal_form::test_conjunctive_normal_form("ABC||!");
    ex06_conjunctive_normal_form::test_conjunctive_normal_form("ABC|&");
    ex06_conjunctive_normal_form::test_conjunctive_normal_form("ABC&|");
    ex06_conjunctive_normal_form::test_conjunctive_normal_form("ABC&|!");
    ex06_conjunctive_normal_form::test_conjunctive_normal_form("ABC^^");
    ex06_conjunctive_normal_form::test_conjunctive_normal_form("ABC>>");


    println!("\nex07:");
    ex07_sat::test_sat("A");
    ex07_sat::test_sat("A!");
    ex07_sat::test_sat("AA|");
    ex07_sat::test_sat("AA&");
    ex07_sat::test_sat("AA!&");
    ex07_sat::test_sat("AA^");
    ex07_sat::test_sat("AB^");
    ex07_sat::test_sat("AB=");
    ex07_sat::test_sat("AA>");
    ex07_sat::test_sat("AA!>");

    ex07_sat::test_sat("ABC||");
    ex07_sat::test_sat("AB&A!B!&&");
    ex07_sat::test_sat("ABCDE&&&&");
    ex07_sat::test_sat("AAA^^");
    ex07_sat::test_sat("ABCDE^^^^");


    println!("\nex08:");
    let mut vec: Vec<i32> = Vec::new();
    ex08_powerset::test_powerset(vec.clone());
    vec.push(21);
    ex08_powerset::test_powerset(vec.clone());
    vec.push(42);
    ex08_powerset::test_powerset(vec.clone());
    vec.push(69);
    ex08_powerset::test_powerset(vec.clone());


    let set_null = vec![
        vec![],
    ];
    let set42 = vec![
        vec![42],
    ];
    let set1 = vec![
        vec![1, 2, 3],
        vec![2, 3, 4],
    ];
    let set2 = vec![
        vec![0, 1, 2],
        vec![],
    ];
    let set3 = vec![
        vec![0, 1, 2],
        vec![0],
    ];
    let set4 = vec![
        vec![0, 1, 2],
        vec![42],
    ];
    let set5 = vec![
        vec![0],
        vec![1, 2],
    ];
    let set6 = vec![
        vec![0],
        vec![0, 1, 2],
    ];
    let set7 = vec![
        vec![],
        vec![],
        vec![],
    ];
    let set8 = vec![
        vec![0],
        vec![1],
        vec![2],
    ];
    let set9 = vec![
        vec![0],
        vec![0],
        vec![0],
    ];
    let set10 = vec![
        vec![0],
        vec![0],
        vec![],
    ];

    println!("\nex09:");
    ex09_eval_set::test_eval_set("A", set_null.clone());
    ex09_eval_set::test_eval_set("A!", set_null.clone());
    ex09_eval_set::test_eval_set("A", set42.clone());
    ex09_eval_set::test_eval_set("A!", set42.clone());
    ex09_eval_set::test_eval_set("A!B&", set1.clone());
    ex09_eval_set::test_eval_set("AB|", set2.clone());
    ex09_eval_set::test_eval_set("AB&", set2.clone());
    ex09_eval_set::test_eval_set("AB&", set3.clone());
    ex09_eval_set::test_eval_set("AB&", set4.clone());
    ex09_eval_set::test_eval_set("AB^", set3.clone());
    ex09_eval_set::test_eval_set("AB>", set5.clone());
    ex09_eval_set::test_eval_set("AB>", set6.clone());

    ex09_eval_set::test_eval_set("ABC||", set7.clone());
    ex09_eval_set::test_eval_set("ABC||", set8.clone());
    ex09_eval_set::test_eval_set("ABC||", set9.clone());
    ex09_eval_set::test_eval_set("ABC&&", set10.clone());
    ex09_eval_set::test_eval_set("ABC&&", set9.clone());
    ex09_eval_set::test_eval_set("ABC^^", set9.clone());
    ex09_eval_set::test_eval_set("ABC>>", set9.clone());


    println!("\nex10:");
    ex10_curve::test_map(0, 0);
    ex10_curve::test_map(65535, 65535);
    ex10_curve::test_map(100, 100);


    println!("\nex11:");
    ex11_inverse_curve::test_reverse_map(0.0);
    ex11_inverse_curve::test_reverse_map(map(0, 0));
    println!("");
    ex11_inverse_curve::test_reverse_map(1.0);
    ex11_inverse_curve::test_reverse_map(map(65535, 65535));
    println!("");
    ex11_inverse_curve::test_reverse_map(0.0000035874545582540926);
    ex11_inverse_curve::test_reverse_map(map(100, 100));

}