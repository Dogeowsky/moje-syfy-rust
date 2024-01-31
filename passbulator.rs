// Brak funkcjonalności z polskimi znakami ze względu na to, że są kodowane na dwóch bajtach i nie mam ochoty bawić się z segmentacją unicode :D

use std::io;
use rand::{Rng, thread_rng};

fn main() {

    println!("Passbulator - prymitywny generator haseł w CLI");
    println!("Maksymalna długość hasła to 8 bitów, tj. 255 znaków.");

    loop{
        let mut passwd_length= String::new();

        println!("\nPodaj długość hasła: (min. 6 znaków, maks. 255 znaków)");
        io::stdin().read_line(&mut passwd_length).unwrap();

        let passwd_length: u8 = match passwd_length.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Podaj poprawną ilość znaków! (min. 6 znaków, maks. 255 znaków)");
                continue;
            }
        };

        if passwd_length < 6 {
            println!("Podaj ilość znaków większą lub równą 6!");
            continue;
        }

        // Zapytanie o wielkie litery w haśle

        let uppercase: bool;

        'uc: loop {
            let mut allow_uppercase = String::new();

            println!("Zezwolić na wielkie litery? (T/N)");
            io::stdin().read_line(&mut allow_uppercase).unwrap();

            
            if allow_uppercase.to_lowercase().trim() == "t" || allow_uppercase.to_lowercase().trim() == "tak" {
                uppercase = true;
                break 'uc;
            }
            else if allow_uppercase.to_lowercase().trim() == "n" || allow_uppercase.to_lowercase().trim() == "nie" {
                uppercase = false;
                break 'uc;
            }
        }

        // Zapytanie o liczby w haśle

        let numbers: bool;

        'num: loop {
            let mut allow_numbers = String::new();

            println!("Zezwolić na liczby? (T/N)");
            io::stdin().read_line(&mut allow_numbers).unwrap();

            
            if allow_numbers.to_lowercase().trim() == "t" || allow_numbers.to_lowercase().trim() == "tak" {
                numbers = true;
                break 'num;
            }
            else if allow_numbers.to_lowercase().trim() == "n" || allow_numbers.to_lowercase().trim() == "nie" {
                numbers = false;
                break 'num;
            } 
        }
        
        // Zapytanie o znaki specjalne

        let specialchars: bool;

        'sc: loop {
            let mut allow_specialchars = String::new();

            println!("Zezwolić na znaki specjalne (/, %, #, @, & itd.)? (T/N)");
            io::stdin().read_line(&mut allow_specialchars).unwrap();

            if allow_specialchars.to_lowercase().trim() == "t" || allow_specialchars.to_lowercase().trim() == "tak" {
                specialchars = true;
                break 'sc;
            }
            else if allow_specialchars.to_lowercase().trim() == "n" || allow_specialchars.to_lowercase().trim() == "nie" {
                specialchars = false;
                break 'sc;
            } 
        }

        // Zapytanie o polskie znaki - nie ma bo nie chce mi się walczyć z dwubajtowym kodowaniem unicode

        /* let polishsigns: bool;

        'sc: loop {
            let mut allow_polishsigns = String::new();

            println!("Zezwolić na polskie znaki (ą, ę, ć, ł, ó itd.)? (T/N)");
            io::stdin().read_line(&mut allow_polishsigns).unwrap();

            if allow_polishsigns.to_lowercase().trim() == "t" || allow_polishsigns.to_lowercase().trim() == "tak" {
                polishsigns = true;
                break 'sc;
            }
            else if allow_polishsigns.to_lowercase().trim() == "n" || allow_polishsigns.to_lowercase().trim() == "nie" {
                polishsigns = false;
                break 'sc;
            }
        }*/

        // Zapytanie o podkreślnik

        let underline: bool;

        'ul: loop {
            let mut allow_underline = String::new();
            
            println!("Zezwolić na podkreślnik (_)? (T/N)");
            io::stdin().read_line(&mut allow_underline).unwrap();

            if allow_underline.to_lowercase().trim() == "t" || allow_underline.to_lowercase().trim() == "tak" {
                underline = true;
                break 'ul;
            }
            else if allow_underline.to_lowercase().trim() == "n" || allow_underline.to_lowercase().trim() == "nie" {
                underline = false;
                break 'ul;
            }
        }

        // Podsumowanie

        println!("\n//// PODSUMOWANIE ////\n// Liczba znaków wynosi: {passwd_length}\n// Zezwolono na wielke litery?: {uppercase}\n// Zezwolono na liczby?: {numbers}\n// Zezwolono na znaki specjalne?: {specialchars}\n// Zezwolono na podkreślniki?: {underline}");
        // \n// Zezwolono na polskie znaki?: {polishsigns}

        // Zbiór dozwolonych znaków

        let mut allowed_chars = String::from("qwertyuiopasdfghjklzxcvbnm");

        if uppercase {
            allowed_chars += "QWERTYUIOPASDFGHJKLZXCVBNM";
        }
        if numbers {
            allowed_chars += "1234567890";
        }
        if specialchars {
            allowed_chars += "!\"$#%'()*+`-./:;<=>?@[\\]^{|}~&";
        }
        /*if polishsigns {
            allowed_chars += "ąćęłńóśźż";
        }
        if polishsigns && uppercase {
            allowed_chars += "ĄĆĘŁŃÓŚŹŻ";
        }*/
        if underline {
            allowed_chars += "_";
        }

        println!("\nZbiór zezwolonych znaków:\n{allowed_chars}");

        // Generowanie hasła

        let mut password = String::new();

        for _ in 0..passwd_length {
            let rand_num = thread_rng().gen_range(0..allowed_chars.len());

            password.push(allowed_chars.chars().nth(rand_num).unwrap());
        }

        println!("\nWygenerowane hasło: {password}\n");

        // Zapytanie o kontynuacje

        'end: loop {
            let mut exit = String::new();

            println!("Czy chcesz kontynuować? (T/N)");
            io::stdin().read_line(&mut exit).unwrap();

            if exit.to_lowercase().trim() == "t" || exit.to_lowercase().trim() == "tak" {
                break 'end;
            }
            else if exit.to_lowercase().trim() == "n" || exit.to_lowercase().trim() == "nie" {
                return;
            }
        }
    }
}

