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
        karty[0][2] = String::from("kulový svršek");
        karty[0][3] = String::from("kulový spodek");
        karty[0][4] = String::from("kulová desítka");
        karty[0][5] = String::from("kulová devítka");
        karty[0][6] = String::from("kulová osmička");
        karty[0][7] = String::from("kulová sedmička");

        karty[1][0] = String::from("srdcové eso");
        karty[1][1] = String::from("srdcový král");
        karty[1][2] = String::from("srdcový svršek");
        karty[1][3] = String::from("srdcový spodek");
        karty[1][4] = String::from("srdcová desítka");
        karty[1][5] = String::from("srdcová devítka");
        karty[1][6] = String::from("srdcová osmička");
        karty[1][7] = String::from("srdcová sedmička");

        karty[2][0] = String::from("listové eso");
        karty[2][1] = String::from("listový král");
        karty[2][2] = String::from("listový svršek");
        karty[2][3] = String::from("listový spodek");
        karty[2][4] = String::from("listová desítka");
        karty[2][5] = String::from("listová devítka");
        karty[2][6] = String::from("listová osmička");
        karty[2][7] = String::from("listová sedmička");

        karty[3][0] = String::from("žaludové eso");
        karty[3][1] = String::from("žaludový král");
        karty[3][2] = String::from("žaludový svršek");
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

fn get_card_color(card: &str) -> &str {
    if card.contains("kulové") || card.contains("kulová") || card.contains("kulový") {
        return "kule";
    } else if card.contains("srdcové") || card.contains("srdcová") || card.contains("srdcový") {
        return "srdce";
    } else if card.contains("listové") || card.contains("listová") || card.contains("listový") {
        return "listy";
    } else if card.contains("žaludové") || card.contains("žaludová") || card.contains("žaludový") {
        return "žaludy";
    } else {
        return "neznámé";
    }
}

fn get_card_value(card: &str) -> &str {
    if card.contains("eso") {
        return "eso";
    } else if card.contains("král") {
        return "král";
    } else if card.contains("svršek") {
        return "svršek";
    } else if card.contains("spodek") {
        return "spodek";
    } else if card.contains("desítka") {
        return "desítka";
    } else if card.contains("devítka") {
        return "devítka";
    } else if card.contains("osmička") {
        return "osmička";
    } else if card.contains("sedmička") {
        return "sedmička";
    } else {
        return "neznámá";
    }
}

fn main() {
    let mut ruka: Vec<String> = Vec::new(); // vytvoření pole karet v ruce hráče
    let mut pořadí = true;

    for _ in 0..4 {
        ruka.push(random_karta());
    }

    

    let mut ruka_protihrac: Vec<String> = Vec::new(); // vytvoření pole karet v ruce protihráče

    for _ in 0..4 {
        ruka_protihrac.push(random_karta());
    }


    let mut polozena_karta = random_karta(); // vytvoření proměnné pro položenou kartu
    


    loop { //game loop
    println!("\nPoložená karta: {}\n", polozena_karta);
    /* 
    println!("\nRuka protihráče:");  //-------------------------Odkomentujte pokud chcete vidět ruku protihráče---------------------------------
    for (index, card) in ruka_protihrac.iter().enumerate() {
        println!("{}: {}", index + 1, card); // vypíše ruku protihráče s indexem
    }
    println!("\n");
    */
    println!("Ruka hráče:");
    for (index, card) in ruka.iter().enumerate() {
            println!("{}: {}", index + 1, card); // vypíše ruku hráče s indexem
        }
    println!("\n");


    loop{
    if ruka.is_empty() {
        println!("Vyhrál jsi!");
        return;
    }
    if ruka_protihrac.is_empty() {
        println!("Protihráč vyhrál!");
        return;
    }

    println!("Chtěl by sis líznout kartu(l), nebo hrát?(h)");
    let volba = input();
    match volba.as_str() { // výběr možnosti
        "l" => {
            ruka.push(random_karta());
            println!("Líznul jsi si kartu: {}", ruka.last().unwrap()); 
            pořadí = !pořadí;
            break;
        }
        "h" => {
            
            println!("Vyber kartu, kterou chceš hrát:");

            let index = input().parse::<usize>().unwrap();
            let selected_card = ruka[index - 1].clone();
            println!("Vybral jsi kartu: {}\n", selected_card);
            if get_card_color(&selected_card) == get_card_color(&polozena_karta) || get_card_value(&selected_card) == get_card_value(&polozena_karta) {
                let card_value = get_card_value(&selected_card).to_string();
                polozena_karta = selected_card;
                ruka.remove(index - 1);

                match card_value.as_str() {
                    "eso" => {
                        // chain představuje počet zahraných es – počítá se i to, které bylo právě odehráno
                        let mut chain = 1;
                        // Nejprve zkontrolujeme, zda protihráč umí nadřadit eso.
                        if let Some(pos) = ruka_protihrac.iter().position(|card| card.contains("eso")) {
                            ruka_protihrac.remove(pos);
                            println!("Hráč hrál eso, protihráč použil vlastní eso.");
                            chain += 1;
                            // Když protihráč nadřadí eso, tah se obrátí k němu.
                            pořadí = !pořadí;
                        } else {
                            println!("Hráč hrál eso, protihráč nemá eso, takže tah zůstává u tebe.");
                            println!("------------------------------------");

                            break;
                        }
                        
                        // Cyklus pro případ, kdy může dojít k dalšímu řetězení (maximálně do 4 es)
                        while chain < 4 {
                            // Zjistíme, zda máš (hráč) ještě eso pro odpověď.
                            if ruka.iter().any(|card| card.contains("eso")) {
                                println!("Chceš použít další eso? (a/n)");
                                let volba = input().to_lowercase();
                                if volba == "a" {
                                    if let Some(pos) = ruka.iter().position(|card| card.contains("eso")) {
                                        ruka.remove(pos);
                                        println!("Hráč použil další eso.");
                                        chain += 1;
                                        // Po zahrání es se tah přepíná k tobě, tedy obracíme pořadí.
                                        
                                    }
                                    // Nyní zkusí protihráč opět nadřadit eso.
                                    if let Some(pos) = ruka_protihrac.iter().position(|card| card.contains("eso")) {
                                        ruka_protihrac.remove(pos);
                                        println!("Protihráč použil další eso.");
                                        chain += 1;
                                        // Tah se opět obrací, nyní opět patří tobě.
                                        
                                    } else {
                                        println!("Protihráč nemá eso, takže tah zůstává u tebe.");
                                        println!("------------------------------------");
                                        break;
                                    }
                                } else {
                                    println!("Rozhodl ses neřídit řetězec, tah zůstává u protihráče.");
                                    println!("------------------------------------");
                                        pořadí = !pořadí;
                                        break;
                                }
                            } else {
                                println!("Nemáš další eso, tah zůstává u protihráče.");
                                println!("------------------------------------");
                                pořadí = !pořadí;
                                break;
                            }
                        }
                        
                        break;
                    }
                    "svršek" => {
                        println!("Hráč hrál svrška, na co se mění barva?");
                        println!("1 - Kule, 2 - Srdce, 3 - Listy, 4 - Žaludy");
                        let barva = input();
                        match barva.as_str() {
                            "1" => {
                                println!("Barva změněna na kule.");
                                polozena_karta = "kulový svršek".to_string();
                                pořadí = !pořadí;
                                break;
                            }
                            "2" => {
                                println!("Barva změněna na srdce.");
                                polozena_karta = "srdcový svršek".to_string();
                                pořadí = !pořadí;
                                break;
                            }
                            "3" => {
                                println!("Barva změněna na listy.");
                                polozena_karta = "listový svršek".to_string();
                                pořadí = !pořadí;
                                break;
                            }
                            "4" => {
                                println!("Barva změněna na žaludy.");
                                polozena_karta = "žaludový svršek".to_string();
                                pořadí = !pořadí;
                                break;
                            }
                            _ => {
                                pořadí = !pořadí;
                                break;
                            }
                        }
                    }
                    "sedmička" => {
                        // chain představuje počet zahraných sedmiček (počítá se i ta, kterou hráč právě zahraje)
                        let mut chain = 1;
                        // Nejprve zkusí protihráč zahrát sedmičku
                        if let Some(pos) = ruka_protihrac.iter().position(|card| card.contains("sedmička")) {
                            ruka_protihrac.remove(pos);
                            println!("Hráč hrál sedmičku, protihráč použil vlastní sedmičku.");
                            chain += 1;
                        } else {
                            println!("Hráč hrál sedmičku, protihráč bere {} karet.", chain * 2);
                            println!("------------------------------------");
                            for _ in 0..(chain * 2) {
                                ruka.push(random_karta());
                            }
                        pořadí = !pořadí;
                        break;
                        }
                        
                        // Cyklus, ve kterém se hráč ptá, zda chce nadřet další sedmičku,
                        // ale celkový počet sedmiček nesmí překročit 4 (penalizace maximálně 8 karet)
                        while chain < 4 {
                            if ruka.iter().any(|card| card.contains("sedmička")) {
                                println!("Chceš použít další sedmičku? (a/n)");
                                let volba = input().to_lowercase();
                                if volba == "a" {
                                    if let Some(pos) = ruka.iter().position(|card| card.contains("sedmička")) {
                                        ruka.remove(pos);
                                        println!("Hráč použil další sedmičku.");
                                        chain += 1;
                                    }
                                    // Nyní zkusí protihráč opět zahrát sedmičku
                                    if let Some(pos) = ruka_protihrac.iter().position(|card| card.contains("sedmička")) {
                                        ruka_protihrac.remove(pos);
                                        println!("Protihráč použil další, vlastní sedmičku.");
                                        chain += 1;
                                    } else {
                                        println!("Protihráč nemá sedmičku a bere {} karet.", chain * 2);
                                        println!("------------------------------------");
                                        for _ in 0..(chain * 2) {
                                            ruka.push(random_karta());
                                        }
                        pořadí = !pořadí;
                        break;
                                    }
                                } else {
                                    println!("Hráč bere {} karet.", chain * 2);
                                println!("------------------------------------");
                                    for _ in 0..(chain * 2) {
                                        ruka.push(random_karta());
                                    }
                        pořadí = !pořadí;
                        break;
                                }
                            } else {
                                println!("Nemáš další sedmičku. Protihráč bere {} karet.", chain * 2);
                                println!("------------------------------------");
                                for _ in 0..(chain * 2) {
                                    ruka.push(random_karta());
                                }
                        pořadí = !pořadí;
                        break;
                            }
                        }
                        
                        // Pokud hráč pokračuje v řetězci až do maximálního počtu sedmiček, pak se penalizace
                        // omezuje na 8 karet (protože 4 * 2 = 8)
                        if chain >= 4 {
                            println!("Hráč již nemá sedmičky a bere 8 karet.");
                            println!("------------------------------------");
                            for _ in 0..8 {
                                ruka.push(random_karta());
                            }
                        }
                        
                        pořadí = !pořadí;
                        break;
                    }
                    _ => {
                        pořadí = !pořadí;
                        break;
                    }
                }
            }
            else {
                println!("Karta není stejné barvy, nemůžeš hrát.");
                continue;
            }
        }
        _ => {
            println!("Neplatná volba!");
        }
    }
    }

    if pořadí == false {
        let mut index = 0;
    
        loop {
            if index >= ruka_protihrac.len() {
                ruka_protihrac.push(random_karta());
                println!("Protihráč líznul kartu: {}\n", ruka_protihrac.last().unwrap());
                println!("------------------------------------");
                pořadí = !pořadí;
                break;
            }
    
            let selected_card = ruka_protihrac[index].clone();
            if get_card_color(&selected_card) == get_card_color(&polozena_karta) || get_card_value(&selected_card) == get_card_value(&polozena_karta) {
                ruka_protihrac.remove(index);
                let card_value = get_card_value(&selected_card).to_string();
                let selected_card_copy = selected_card.clone();
                polozena_karta = selected_card;
                match card_value.as_str() {
                    "eso" => {
                        // chain představuje počet zahraných es – počítá se i to, které protihráč právě zahrál
                        let mut chain = 1;
                        // Nejprve ověříme, zda hráč umí odpovědět vlastním esem.
                        println!("Protihráč hrál eso.");
                        if let Some(pos) = ruka.iter().position(|card| card.contains("eso")) {
                            println!("Chceš použít své eso? (a/n)");
                            let volba = input().to_lowercase();
                            if volba == "a" {
                                ruka.remove(pos);
                                println!("Hráč použil své eso.");
                                chain += 1;
                                // Přepnuti tahu na hráče.
                                pořadí = !pořadí;
                            }
                            else {
                                println!("Hráč se rozhodl eso nehrát, tah zůstává u protihráče.");
                                println!("------------------------------------");
                                break;
                            }
                        } else {
                            println!("Hráč nemá eso, takže tah zůstává u protihráče.");
                            println!("------------------------------------");
                            break;
                        }
                        
                        // Řetězení – PC reaguje automaticky, ale hráče se vždy ptáme.
                        while chain < 4 {
                            // PC odpovídá automaticky: pokud má další eso, vždy ho použije.
                            if let Some(pos) = ruka_protihrac.iter().position(|card| card.contains("eso")) {
                                ruka_protihrac.remove(pos);
                                println!("Protihráč použil další eso.");
                                chain += 1;
                                // Tah se přepne na hráče.
                                pořadí = !pořadí;
                            } else {
                                println!("Protihráč nemá další eso, tah zůstává u hráče.");
                                println!("------------------------------------");
                                break;
                            }
                            
                            // Hráč je opět dotázán, zda chce pokračovat v řetězci.
                            if ruka.iter().any(|card| card.contains("eso")) {
                                println!("Chceš použít další eso? (a/n)");
                                let volba = input().to_lowercase();
                                if volba == "a" {
                                    if let Some(pos) = ruka.iter().position(|card| card.contains("eso")) {
                                        ruka.remove(pos);
                                        println!("Hráč použil další eso.");
                                        chain += 1;
                                        // Po zahrání ese se tah přepne zpět na protihráče.
                                        pořadí = !pořadí;
                                    }
                                } else {
                                    println!("Hráč se rozhodl eso nehrát, tah zůstává u protihráče.");
                                    println!("------------------------------------");
                                    break;
                                }
                            } else {
                                println!("Hráč nemá další eso, tah zůstává u protihráče.");
                                println!("------------------------------------");
                                break;
                            }
                        }
                        
                        break;
                    }
                    
                    "svršek" => {
                        let mut rng = rand::thread_rng();
                        let nahoda = rng.gen_range(0..ruka_protihrac.len());
                        let barva = get_card_color(&ruka_protihrac[nahoda]);
                        match barva {
                            "kule" => {
                                println!("Protihráč hrál svrška, barva změněna na kule.");
                                polozena_karta = "kulový svršek".to_string();
                                println!("------------------------------------");
                                pořadí = !pořadí;
                                break;
                            }
                            "srdce" => {
                                println!("Protihráč hrál svrška, barva změněna na srdce.");
                                polozena_karta = "srdcový svršek".to_string();
                                println!("------------------------------------");
                                pořadí = !pořadí;
                                break;
                            }
                            "listy" => {
                                println!("Protihráč hrál svrška, barva změněna na listy.");
                                polozena_karta = "listový svršek".to_string();
                                println!("------------------------------------");
                                pořadí = !pořadí;
                                break;
                            }
                            "žaludy" => {
                                println!("Protihráč hrál svrška, barva změněna na žaludy.");
                                polozena_karta = "žaludový svršek".to_string();
                                println!("------------------------------------");
                                pořadí = !pořadí;
                                break;
                            }
                            _ => {
                                println!("------------------------------------");
                                pořadí = !pořadí;
                                break;
                            }
                        }
                    }
                    "sedmička" => {
                        // chain představuje počet zahraných sedmiček – počítá se i ta, kterou protihráč právě zahrál
                        let mut chain = 1;
                        // Sedmičku inicializoval protihráč, nyní hráče musí reagovat:
                        if let Some(pos) = ruka.iter().position(|card| card.contains("sedmička")) {
                            println!("Chceš použít svoji sedmičku? (a/n)");
                            let volba = input().to_lowercase();
                            if volba == "a" {
                                ruka.remove(pos);
                                println!("Hráč použil svoji sedmičku.");
                                chain += 1;
                                // Přepnuti tahu na hráče.
                                
                            }
                            else {
                                println!("Protihráč hrál sedmičku, hráč se rozhodl sedmičku nehrát a bere {} karty.", chain * 2);
                                println!("------------------------------------");
                                for _ in 0..(chain * 2) {
                                    ruka.push(random_karta());
                                }
                                pořadí = !pořadí;
                                break;
                            }
                            // Přepnutí tahu na protihráče zatím neproběhne – pokračujeme v řetězci
                        } else {
                            println!("Protihráč hrál sedmičku, hráč nemá sedmičku a bere {} karty.", chain * 2);
                            println!("------------------------------------");
                            for _ in 0..(chain * 2) {
                                ruka.push(random_karta());
                            }
                            
                                pořadí = !pořadí;
                                break;
                        }
                        
                        // Řetěz se pokračuje maximálně do 4 sedmiček (tedy penalizace max. 8 karet)
                        while chain < 4 {
                            // Nejprve PC odpovídá automaticky, pokud má další sedmičku
                            if let Some(pos) = ruka_protihrac.iter().position(|card| card.contains("sedmička")) {
                                ruka_protihrac.remove(pos);
                                println!("Protihráč použil další sedmičku.");
                                chain += 1;
                            } else {
                                println!("Protihráč nemá další sedmičku a bere {} karet.", chain * 2);
                            println!("------------------------------------");
                                for _ in 0..(chain * 2) {
                                    ruka_protihrac.push(random_karta());
                                }
                                pořadí = !pořadí;
                                break;
                            }
                            
                            // Nyní se hráče ptáme, zda chce odpovědět vlastní sedmičkou
                            if ruka.iter().any(|card| card.contains("sedmička")) {
                                println!("Chceš použít další sedmičku? (a/n)");
                                let volba = input().to_lowercase();
                                if volba == "a" {
                                    if let Some(pos) = ruka.iter().position(|card| card.contains("sedmička")) {
                                        ruka.remove(pos);
                                        println!("Hráč použil další sedmičku.");
                                        chain += 1;
                                    }
                                } else {
                                    println!("Hráč bere {} karet.", chain * 2);
                            println!("------------------------------------");
                                    for _ in 0..(chain * 2) {
                                        ruka.push(random_karta());
                                    }
                                    
                                pořadí = !pořadí;
                                break;
                                }
                            } else {
                                println!("Hráč nemá další sedmičku, bere {} karet.", chain * 2);
                            println!("------------------------------------");
                                for _ in 0..(chain * 2) {
                                    ruka.push(random_karta());
                                }
                                pořadí = !pořadí;
                                break;
                            }
                        }
                        
                        // Pokud řetězec dosáhne maxima, hráč dostane penalizaci 8 karet
                        if chain >= 4 {
                            println!("Protihráč už nemá více sedmiček a bere 8 karet.");
                            println!("------------------------------------");
                            for _ in 0..8 {
                                ruka_protihrac.push(random_karta());
                            }
                        }
                        
                        pořadí = !pořadí;
                        break;
                    }
                    _ => {
                        println!("Protihráč hrál kartu: {}\n", &selected_card_copy);
                        println!("------------------------------------");
                        pořadí = !pořadí;
                        break;
                    }
                }
                }
             
            else {
                index += 1;
            }
        }
    }
  }
}

