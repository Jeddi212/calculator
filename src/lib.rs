use std::io;

pub fn start() -> String {

    loop {

        print!("Continue? (yes): ");
        if is_close() {
            continue;
        }
        break "Thank You".to_owned();
    }
}

fn is_close() -> bool {
    
    let mut close = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut close).expect("error: unable to read user input");

    match close.trim().to_lowercase().as_str() {
        "yes" => true,
        _ => false,
    }
}