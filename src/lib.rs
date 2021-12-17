use std::io;

pub fn start() -> String {

    loop {

        print!("Continue? (yes): ");
        if stop() {
            continue;
        }
        break "Thank You".to_owned();
    }
}

fn stop() -> bool {
    
    let mut close = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut close).expect("error: unable to read user input");

    match close.trim().to_lowercase().as_str() {
        "yes" => true,
        _ => false,
    }
}