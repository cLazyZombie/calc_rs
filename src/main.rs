use std::io::{self, Write};

mod calculator;


fn main() {

    loop {
        print!("enter equation: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let result = calculator::calculate(&input);
                println!("result: {}", result);
            }
            Err(err) => {
                println!("error. {}", err);
                return;
            }
        }
    }
}
