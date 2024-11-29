use std::io::Write;
use std::io;
use std::fs::File;
use std::io::BufRead;

fn input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Chyba při čtení řádku");
    input
}

fn main() {
    println!("Kolik zápisů bude? ");
    let pocet = input()
        .trim()
        .parse::<i32>()
        .expect("Zadejte platné číslo!"); // Parse input as an integer

    let mut array: Vec<String> = Vec::new(); // Dynamic array for tasks

    // Collect tasks
    for _ in 0..pocet {
        println!("Zadejte úkol: ");
        let udelat = input();
        array.push(udelat.trim().to_string()); // Trim and save each task
    }

    loop {
        println!("\nVaše úkoly:");
        for (index, element) in array.iter().enumerate() {
            println!("{}: {}", index + 1, element); // Display tasks with indices
        }

        println!(
            "Chtěli byste zápisy upravit(u), smazat(s), přidat(p), uložit(l) nebo vložit(v)?"
        );
        let _input = input().trim().to_lowercase();

        if _input == "u" {
            println!("Který zápis chcete upravit? ");
            let index = input()
                .trim()
                .parse::<usize>()
                .expect("Zadejte platné číslo!") - 1;
            println!("Zadejte nový zápis: ");
            let novy = input();
            array[index] = novy.trim().to_string();
            } 

        else if _input == "s" {
            println!("Který zápis chcete smazat? ");
            let index = input()
                .trim()
                .parse::<usize>()
                .expect("Zadejte platné číslo!") - 1;
            array.remove(index);
            } 

        else if _input == "p" {
            println!("Zadejte nový zápis: ");
            let novy = input();
            array.push(novy.trim().to_string());
            } 
        
        else if _input == "l" {
            println!("Zadejte název souboru: ");
            let filename = input().trim().to_string();
            match File::create(filename.clone()) {
                Ok(mut file) => {
                    for task in &array {
                        if let Err(e) = writeln!(file, "{}", task) {
                            println!("Chyba při zápisu do souboru: {}", e);
                            break;
                        }
                    }
                    println!("Úkoly byly úspěšně uloženy do souboru: {}", filename);
                }
                Err(e) => println!("Chyba při vytváření souboru: {}", e),
            }
        }

        else if _input == "v" {
            println!("Zadejte název souboru: ");
            let filename = input().trim().to_string();
            match File::open(filename.clone()) {
                Ok(file) => {
                    let reader = io::BufReader::new(file);
                    for line in reader.lines() {
                        match line {
                            Ok(task) => array.push(task),
                            Err(e) => {
                                println!("Chyba při čtení souboru: {}", e);
                                break;
                            }
                        }
                    }
                    println!("Úkoly byly úspěšně načteny ze souboru: {}", filename);
                }
                Err(e) => println!("Chyba při otevírání souboru: {}", e),
            }
        }


        else {
            println!("Neplatný vstup, zkuste to znovu.\n");
        }
    }
}

