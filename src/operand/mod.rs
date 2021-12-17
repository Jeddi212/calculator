use std::io;

struct Operand {
    num1: f64,
    num2: f64,
}

impl Operand {
    fn addition(&self, num1: f64, num2: f64) -> f64 {
        num1 + num2
    }
    
    fn substraction(&self, num1: f64, num2: f64) -> f64 {
        num1 - num2
    }
    
    fn product(&self, num1: f64, num2: f64) -> f64 {
        num1 * num2
    }
    
    fn division(&self, num1: f64, num2: f64) -> f64 {
        num1 / num2
    }
    
    fn modulus(&self, num1: f64, num2: f64) -> f64 {
        num1 % num2
    }
    
    fn power(&self, num1: f64, num2: f64) -> f64 {
        num1.powf(num2)
    }
    
    fn squro(&self, num1: f64) -> f64 {
        num1.sqrt()
    }
}

pub fn calculate() -> () {
    
    let result_number: f64;
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut op = String::new();
    let stdin = io::stdin();

    println!("Num 1 : ");
    stdin.read_line(&mut num1).expect("error read input");

    println!("Num 2 : ");
    stdin.read_line(&mut num2).expect("error read input");

    let num1: f64 = num1.to_string().trim().parse().expect("The input should a number");

    let num2: f64 = num2.to_string().trim().parse().expect("The input should a number");

}
