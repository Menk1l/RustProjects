//0 - kule, 1 - srdce, 2 - listy, 3 - zaludy
//0 - eso, 1 - kral, 2 - svrsek, 3 - spodek 4 - desitka, 5 - devitka, 6 - osmicka, 7 - sedmicka

#[macro_use]
extern crate lazy_static;

use rand::Rng;
use std::sync::Mutex;
use std::io;


lazy_static! {
    static ref KARTY: Vec<Vec<String>> = { // vytvoření "globální" pole karet
        let mut karty = vec![vec![String::new(); 8]; 4];

        karty[0][0] = String::from("kulové eso");
        karty[0][1] = String::from("kulový král");
        karty[0][2] = String::from("kulový svrsek");
        karty[0][3] = String::from("kulový spodek");
        karty[0][4] = String::from("kulová desítka");
        karty[0][5] = String::from("kulová devítka");
        karty[0][6] = String::from("kulová osmička");
        karty[0][7] = String::from("kulová sedmička");

        karty[1][0] = String::from("srdcové eso");
        karty[1][1] = String::from("srdcový král");
        karty[1][2] = String::from("srdcový svrsek");
        karty[1][3] = String::from("srdcový spodek");
        karty[1][4] = String::from("srdcová desítka");
        karty[1][5] = String::from("srdcová devítka");
        karty[1][6] = String::from("srdcová osmička");
        karty[1][7] = String::from("srdcová sedmička");

        karty[2][0] = String::from("listové eso");
        karty[2][1] = String::from("listový král");
        karty[2][2] = String::from("listový svrsek");
        karty[2][3] = String::from("listový spodek");
        karty[2][4] = String::from("listová desítka");
        karty[2][5] = String::from("listová devítka");
        karty[2][6] = String::from("listová osmička");
        karty[2][7] = String::from("listová sedmička");

        karty[3][0] = String::from("žaludové eso");
        karty[3][1] = String::from("žaludový král");
        karty[3][2] = String::from("žaludový svrsek");
        karty[3][3] = String::from("žaludový spodek");
        karty[3][4] = String::from("žaludová desítka");
        karty[3][5] = String::from("žaludová devítka");
        karty[3][6] = String::from("žaludová osmička");
        karty[3][7] = String::from("žaludová sedmička");

        return karty;
    };

    static ref SELECTED_CARDS: Mutex<Vec<String>> = Mutex::new(Vec::new()); // vytvoření "globální" pole již vybraných karet
}

fn random_karta() -> String { // funkce pro náhodný výběr karty
    let mut rng = rand::thread_rng();
    let mut selected_cards = SELECTED_CARDS.lock().unwrap();

    loop {
        let nahodne_cislo = rng.gen_range(0..4); // náhodný výběr barvy karty
        let nahodne_cislo2 = rng.gen_range(0..8); // náhodný výběr hodnoty karty
        let card = KARTY[nahodne_cislo][nahodne_cislo2].clone(); 

        // zkontroluje, zda již nebyla karta vybrána
        if !selected_cards.contains(&card) {
            selected_cards.push(card.clone());
            return card;
        }
    }
}

fn input() -> String {
    loop {
        let mut input = String::new();

        // Vyčte řádek z terminálu
        if io::stdin().read_line(&mut input).is_err() {
            println!("Chyba při čtení řádku, zkuste to znovu.");
            continue; // restartuje cyklus pokud dojde k chybě
        }

        // Vrátí vstup bez "neviditelných" znaků na začátku a na konci
        return input.trim().to_string();
    }
}


fn main() {
    let mut ruka: Vec<String> = Vec::new(); // vytvoření pole karet v ruce hráče

    for _ in 0..4 {
        ruka.push(random_karta());
    }

    

    let mut ruka_protihrac: Vec<String> = Vec::new(); // vytvoření pole karet v ruce protihráče

    for _ in 0..4 {
        ruka_protihrac.push(random_karta());
    }

    println!("\nRuka protihráče:");
    for card in &ruka_protihrac {
        println!("{}", card);
    }

    let mut polozena_karta = random_karta(); // vytvoření proměnné pro položenou kartu
    println!("\nPoložená karta: {}", polozena_karta);


    loop {
    println!("Ruka hráče:");
    for (index, card) in ruka.iter().enumerate() {
            println!("{}: {}", index + 1, card); // vypíše ruku hráče s indexem
        }

    println!("Chtěl by sis líznout kartu(l), nebo hrát?(h)");
    let volba = input();

    match volba.as_str() { // výběr možnosti
        "l" => {
            ruka.push(random_karta());
            println!("Líznul jsi si kartu: {}", ruka.last().unwrap()); 
        }
        "h" => {
            println!("Vyber kartu, kterou chceš hrát:");

            let index = input().parse::<usize>().unwrap();
            let selected_card = ruka.remove(index - 1);
            println!("Vybral jsi kartu: {}\n", selected_card);

            
        }
        _ => {
            println!("Neplatná volba!");
        }
    }
  }
}

