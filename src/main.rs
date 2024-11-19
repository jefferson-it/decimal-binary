use std::{
    io::{self, stdout, Write},
    process::exit,
};

enum Action {
    ToDecimal,
    ToBinary,
    ToClose,
}

fn ask_num() -> (String, i32) {
    print!("Digite um número: \n > ");
    stdout().flush().expect("Failed to flush stdout");
    
    let mut num_target = String::new();
    let _ = io::stdin().read_line(&mut num_target);
    num_target = num_target.trim().to_owned();

    let num_to_convert = match num_target.parse::<i32>() {
        Err(_) => {
            println!("O valor digitado não é um número!");
            exit(1)
        },
        Ok(num) => num,
    };

    (num_target, num_to_convert)
}

fn main() {
    let mut num_to_convert: i32 = 0;
    let mut action: Action;

    loop {
        print!("Digite a ação:\n[0] -> Converter para Decimal\n[1] -> Converter para Binário\n[qualquer] -> Encerrar\n > ");
        stdout().flush().expect("Failed to flush stdout");

        let mut num_action = String::new();
        let mut num_target = String::new();

        let _ = io::stdin().read_line(&mut num_action);

        action = match num_action.trim() {
            "0" => {
                (num_target, num_to_convert) = ask_num();
                Action::ToDecimal
            },
            "1" => {
                (num_target, num_to_convert) = ask_num();
                Action::ToBinary
            },
            _ => Action::ToClose,
        };

        match action {
            Action::ToClose => {
                println!("Programa encerrado");
                exit(0)
            }
            Action::ToDecimal => {
                if num_to_convert == 0 {
                    println!("0 em decimal é 0");
                    exit(0)
                }
                let mut res = 0;
                let mut ind = 0;
                let mut num;

                for digit in num_target.to_string().chars().rev() {
                    match String::from(digit).parse::<i32>() {
                        Err(_) => panic!("O valor digitado não é um número!"),
                        Ok(dig) => num = dig,
                    };

                    res += i32::pow(2, ind) * num;

                    ind += 1;
                }

                println!("{num_to_convert} em binário é {res}");
            }
            Action::ToBinary => {
                let mut res = String::new();
                let mut num = num_to_convert;

                if num_to_convert == 0 {
                    println!("0 em binário é 0");
                    exit(0)
                }

                while num > 0 {
                    res.push_str(&(num % 2).to_string());

                    num /= 2;
                }

                res = res.chars().rev().collect();

                if res.len() < 2 {
                    res = format!("0{}", res);
                }

                println!("{num_to_convert} em binário é {res}");
            }
        }

        println!("");
    }
}
