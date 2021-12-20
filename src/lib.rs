mod operand;

use std::io;

pub fn start() -> String {

    let mut result_number = 0_f64;
    let mut user_choose = 2_i8;
    let mut input = String::new();
    let stdin = io::stdin();

    loop {

        // choose Operand
        print_operand();
        
        input.clear();
        stdin.read_line(&mut input).expect("error read input");

        input = input.trim().to_lowercase();

        // do Calculate
        if input == "1" || 
            input == "2" || 
            input == "3" || 
            input == "4" || 
            input == "5" || 
            input == "6" 
        {

            match user_choose {
                1 => result_number = operand::calculate_1(&input, result_number),
                2 => result_number = operand::calculate_2(&input),
                _ => println!("Is you choose the right number option ?")
            }
            println!("Result : {}\n", &result_number);

        } 
        else 
        {
            
            println!("Please input the operand order!");

        }

        // re-use Calculator?
        println!("Continue ?
    1. Use last result
    2. Reset
    3. Quit");
        
        input.clear();
        stdin.read_line(&mut input).expect("error read input");

        input = input.trim().to_lowercase();

        if keep_going(&input, &mut user_choose) {
            continue
        } 
        break "Thank You".to_owned();
    }

    
}

fn print_operand() {

    println!(r#"
        Please input The Operand : 
        1. Addition (+)
        2. Substracting (-)
        3. Product (*)
        4. Division (/)
        5. Modulus (%)
        6. Power (^)
        7. Exit
    "#);

}

fn keep_going(input: &str, user_choose: &mut i8) -> bool {

    match input {
        "1" => {
            *user_choose = 1;
            true
        },
        "2" => {
            *user_choose = 2;
            true
        },
        _ => {
            *user_choose = 3;
            false
        },
    }

}