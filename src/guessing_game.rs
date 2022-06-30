use std::cmp::Ordering;
use std::io;

use rand::Rng;

pub fn exec() -> () {
    let mut rng = rand::thread_rng();
    let generated_number = rng.gen_range(0..101);

    loop {
        println!("Digite um número");
        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&generated_number) {
            Ordering::Less => println!("Muito baixo"),
            Ordering::Greater => println!("Muito alto"),
            Ordering::Equal => {
                println!("Parabéns, você acertou!");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec() {
        assert_eq!(exec(), ());
    }
}