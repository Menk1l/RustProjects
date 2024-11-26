use std::io;

fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Chyba při čtení řádku");
    return input;
}

fn main() {
    println!("Kolik zápisů bude? ");
    let pocet = input().trim().parse::<i32>().unwrap(); //parse::<i32>() převede string na číslo, unwrap() odstraní chybu, pokud je nějaká chyba, program spadne
                                                        //trim() odstraní bílé znaky na začátku a na konci řetězce
    
    let mut array: Vec<String> = Vec::new(); //pole nemůže mít proměnlivou velikost, proto používáme Vector                  

    for _  in  0..pocet {
        println!("Zadejte úkol: ");
        let udelat = input();
        array.push(udelat);  //push() přidá prvek na konec pole
    }
    loop {
        
        for (index, element) in array.iter().enumerate() { //.enumerate() vrací dvojici (index, element). .enumerate() je metoda co vrací index k elementu 
                                                          //Pomocí .iter() se dostaneme k prvkům v poli

        print!("{}: ", index + 1);
        println!("{}", element);   //element je "proměná", která vleze do pole a zkopíruje hodnotu

        }


        

    println!("Chtěli byste zápisy upravit(u), smazat(s) nebo přidat(p)? ");
        let mut _input = input().trim().to_lowercase(); 

    if _input == "u" {
        println!("Který zápis chcete upravit? ");
        let index = input().trim().parse::<usize>().unwrap() - 1;  //pole musí mít index usize (usize nemá záporné číslo)                                                         
                                                                                                                                    
        println!("Zadejte nový zápis: ");
        let novy = input();
        array[index] = novy;
        continue;    

        }
    else if _input == "s" {
        println!("Který zápis chcete smazat? ");
        let index = input().trim().parse::<usize>().unwrap() - 1;  //pole musí mít index usize (usize nemá záporné číslo)
        array.remove(index);                                     //remove() odstraní prvek z pole a posune všechny prvky za ním o jedno místo doleva
        continue; 
        }
    else if _input == "p" {
        println!("Zadejte nový zápis: ");
        let novy = input();
        array.push(novy);
        continue;
        }
    else {
        println!("Neplatný vstup, zkuste to znovu.\n");    
        continue;
        }
    }
}
