mod operand;

use std::io;

pub fn start() -> String {

    let mut result_number :f64;
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

            result_number = operand::calculate(&input);
            println!("Result : {}\n", &result_number);

        } 
        else 
        {
            
            println!("Please input the operand order!");

        }

        // re-use Calculator?
        println!("Continue (yes) ?");
        
        input.clear();
        stdin.read_line(&mut input).expect("error read input");

        input = input.trim().to_lowercase();

        if stop(&input) {
            continue;
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

fn stop(input: &str) -> bool {

    match input {
        "yes" => true,
        _ => false,
    }

}