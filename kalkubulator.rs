use std::io;
// use winconsole::console;

fn main() {

    // & - ampersant
    // * - asterisk

    // console::set_title("Kalkubulator - kalkulator prymitywny").unwrap();

    println!("Kalkulator prymitywny - można wykonywać: dodawanie(+), odejmowanie(-), mnożenie(*), dzielenie(/), wyliczać reszte(%), potęgowanie(^) oraz pierwiastkowanie(v) na 2-ch wartościach 64-bitowych.");
    println!("!! W przypadku potęgowania i pierwiastkowania druga wartość jest stopniem potęgi/pierwiastka. !!");

    'outer: loop {
                
        let (mut a, mut b, mut c) = (String::new(), String::new(), String::new());

        println!("\nWprowadź pierwszą wartość:");
        io::stdin().read_line(&mut a).unwrap();

        let a: f64 = match a.trim().parse() {
            Ok(num) => num,
            Err(_) => { 
                println!("\n// Wprowadź poprawną wartość! \n"); 
                continue 
            },
        };

        println!("Wprowadź drugą wartość:");
        io::stdin().read_line(&mut b).unwrap();
        
        let b: f64 = match b.trim().parse() {
            Ok(num) => num,
            Err(_) => { 
                println!("\n// Wprowadź poprawną wartość! \n"); 
                continue 
            },
        };

        println!("Wprowadź znak operacji:");
        io::stdin().read_line(&mut c).unwrap();

        if b == 0.0 && c.trim() == "/" {
            println!("\n// NIE można dzielić przez 0! \n");
            continue
        }

        if b <= 0.0 && c.trim().to_lowercase() == "v" {
            println!("\n// NIE ma pierwiastka o stopniu równym lub mniejszym niż 0! \n");
            continue
        };

        if b <= 0.0 && c.trim().to_lowercase() == "%" {
            println!("\n// NIE można wyliczyć reszty z dzielenia przez 0! \n");
            continue
        };

        let out: f64;

        match c.trim() {
            "+" => out = a + b,
            "-" => out = a - b,
            "*" => out = a * b,
            "/" => out = a / b,
            "%" => out = a % b,
            "^" => out = a.powf(b),
            "v" | "V" => {
                if a < 0.0 {
                    println!("\n// NIE można pierwiastkować liczb ujemnych! (Poniżej wynik przy użyciu wartości bezwględnej: \n")
                }
                out = f64::powf(a.abs(), 1.0 / b)
            },
            _ => { 
                println!("\n// Wprowadź poprawny znak (+, -, *, /, %, ^, v). \n"); 
                continue 
            }
        };

        println!("\nWynik operacji wynosi: {out} \n");

        loop {
            println!("\n// Czy chcesz kontynuować? (T/N)");
            
            let mut exit = String::new();

            io::stdin().read_line(&mut exit).unwrap();

            let exit = exit.trim();

            if exit.to_lowercase() == "t" || exit.to_lowercase() == "tak" {
                continue 'outer;
            }
            
            else if exit.to_lowercase() == "n" || exit.to_lowercase() == "nie" {
                return;
            }
        }
    }
}

// popierdoli mnie uwu
