use std::io;

pub fn start() -> String {

    let mut input = String::new();
    let stdin = io::stdin();

    loop {

        print_operand();
        
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

    println!("
Please input The Operand : 
1. Addition (+)
2. Substracting (-)
3. Product (*)
4. Division (/)
5. Modulus (%)
6. Power (^)
7. Square Root (√)
8. Exit
");

}

fn stop(input: &str) -> bool {

    assert_eq!("yes", input);

    match input {
        "yes" => true,
        _ => false,
    }

}