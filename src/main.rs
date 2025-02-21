use rand::Rng;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Write};

fn citeste_fisier() -> Result<String, io::Error> {
    let siruri_fisier = fs::read_to_string("src/Dictionar.txt")?;
    Ok(siruri_fisier)
}

fn input() -> String {
    let mut aux = String::new();
    let mut msg = String::new();
    msg.clear();
    aux.clear();
    io::stdin()
        .read_line(&mut aux)
        .expect("Problema la citirea input");
    msg = aux.trim().to_string();

    msg
}

fn help() {
    println!("  HELP \n");
    println!("Pentru a introduce o comanda, tastati numele sau numarul comenzii care va apare in meniu, tastarea unei alte comenzi, inafara de cele existente, va duce la o comanda invalida");
    println!("Pentru a alege o categorie pentru cuvant, tastati denumirea categoriei\n");
    println!("  Comenzi: ");
    println!("'Start': vei intra in meciul de incepere al jocului ");
    println!("'Help': iti ofera informatii care te vor ajuta ");
    println!("'Quit' te scoate dinprogram \n");
    println!("  Reguli: ");
    println!("Dupa ce alegi optiunea de a incepe, trebuie sa-ti alegi categoria din care vrei sa primesti cuvantul");
    println!("Dupa aceea incepe joucl si trebuie sa introduci numai cate o litera, orice nerespectare a regulii iti va aduce un mesaj 'Comanda invalida'");
    println!("Pentru a castiga trebuie sa completezi tot cuvantul introducand pe rand literele");
    println!("La final vei putea vedea scorul obtinut in fisierul Scor.txt");
    println!("Cu cat sunt mai multe litere in cuvant cu atat vei avea sansa de a obtinute un scor mai mare");
    println!("HAVE FUN!");
}

fn main() -> io::Result<()> {
    let mut joc = 0;
    let mut program = 1;
    let mut random;
    let mut cuv_dictionar = Vec::new();
    let mut incepe_joc = 0;
    let mut cuv_joc = String::new();

    while program == 1 {
        while joc == 0 {
            println!("\n  Alegeti optiunea");
            println!("1. Start");
            println!("2. Help");
            println!("3. Quit");

            let comanda = input();

            if comanda == "Start" || comanda == "1" {
                joc = 1;
            } else if comanda == "Help" || comanda == "2" {
                help();
            } else if comanda == "Quit" || comanda == "3" {
                program = 0;
                if program == 0 {
                    std::process::exit(0);
                }
            } else {
                println!("Comanda invalida\n");
            }
        }
        while joc == 1 {
            let mut sectiune_sport = false;
            let mut sectiune_gramatica = false;
            let mut sectiune_matematica = false;
            let mut sectiune_geografie = false;

            println!("\n  Alegeti categoria cuvantului: ");
            println!("1. Sport");
            println!("2. Gramatica");
            println!("3. Matematica");
            println!("4. Geografie\n");
            println!("5. Help");
            println!("6. Quit");

            let comanda = input();

            random = rand::thread_rng();
            let numar_random: u32 = random.gen_range(1..=7);

            if comanda == "Sport" || comanda == "1" {
                let dictionar = citeste_fisier().expect("Eroare la citirea fisierului");

                let mut sectiune = false;
                let mut elem_vector = 0;
                cuv_dictionar.clear();

                for linie in dictionar.lines() {
                    if sectiune {
                        cuv_dictionar.push(linie.trim().to_string());
                        elem_vector += 1;

                        if elem_vector == 7 {
                            break;
                        }
                    }

                    if linie == "Sport:" {
                        sectiune = true;
                        sectiune_sport = true;
                    }
                }
            } else if comanda == "Gramatica" || comanda == "2" {
                let dictionar = citeste_fisier().expect("Eroare la citirea fisierului");

                let mut sectiune = false;
                let mut elem_vector = 0;
                cuv_dictionar.clear();

                for linie in dictionar.lines() {
                    if sectiune {
                        cuv_dictionar.push(linie.trim().to_string());
                        elem_vector += 1;

                        if elem_vector == 7 {
                            break;
                        }
                    }

                    if linie == "Gramatica:" {
                        sectiune = true;
                        sectiune_gramatica = true;
                    }
                }
            } else if comanda == "Matematica" || comanda == "3" {
                let dictionar = citeste_fisier().expect("Eroare la citirea fisierului");

                let mut sectiune = false;
                let mut elem_vector = 0;
                cuv_dictionar.clear();

                for linie in dictionar.lines() {
                    if sectiune {
                        cuv_dictionar.push(linie.trim().to_string());
                        elem_vector += 1;

                        if elem_vector == 7 {
                            break;
                        }
                    }

                    if linie == "Matematica:" {
                        sectiune = true;
                        sectiune_matematica = true;
                    }
                }
            } else if comanda == "Geografei" || comanda == "4" {
                let dictionar = citeste_fisier().expect("Eroare la citirea fisierului");

                let mut sectiune = false;
                let mut elem_vector = 0;
                cuv_dictionar.clear();

                for linie in dictionar.lines() {
                    if sectiune {
                        cuv_dictionar.push(linie.trim().to_string());
                        elem_vector += 1;

                        if elem_vector == 7 {
                            break;
                        }
                    }

                    if linie == "Geografie:" {
                        sectiune = true;
                        sectiune_geografie = true;
                    }
                }
            } else if comanda == "Help" || comanda == "5" {
                help();
            } else if comanda == "Quit" || comanda == "6" {
                joc = 0;
                if joc == 0 {
                    std::process::exit(0);
                }
            } else {
                println!("Comanda invalida!");
            }

            if incepe_joc == 0 {
                cuv_joc = cuv_dictionar
                    .get((numar_random - 1) as usize)
                    .map_or_else(String::default, |s| s.to_string());

                if cuv_joc.is_empty() {
                    println!("Introdu categoria cuvintelor!");
                } else {
                    incepe_joc = 1;
                }
            }
            if incepe_joc == 1 {
                println!("\n    Am intrat in joc");

                let mut cuv__ = String::new();
                let mut litere_folosite = String::new();
                let mut cuv_final = Vec::new();
                let mut litera: String;
                let mut exista = false;
                let mut castigat = 0;
                
                for caracter in cuv_joc.chars() {
                    cuv_final.push(caracter);
                    cuv__.push('_');
                }
                
                let mut incercari_gresite = 0;

                while incercari_gresite < (cuv_joc.len() / 2 + 2) {
                    print!("\nCategoria: ");

                    if sectiune_sport {
                        println!("Sport");
                    } else if sectiune_gramatica {
                        println!("Gramatica");
                    } else if sectiune_matematica {
                        println!("Matematica");
                    } else if sectiune_geografie {
                        println!("Geografie");
                    }

                    println!("Incercari gresite: {}", incercari_gresite);
                    println!("Incercari gresite permise: {}\n", cuv_joc.len() / 2 + 2);
                    println!("Lista de litere folosite: {}\n", litere_folosite);
                    println!("Cuvantul: {}\n", cuv__);
                    println!("Litera dorita: ");

                    litera = input();

                    println!("\n");

                    if litera.len() > 1
                        || litera.is_empty()
                        || litera < 'a'.to_string()
                        || litera > 'z'.to_string()
                    {
                        println!("Input invalid!");
                    } else {
                        litere_folosite.push(' ');
                        litere_folosite.push_str(&litera);

                        for (i, caracter) in cuv_final.iter().enumerate() {
                            if caracter.to_string() == litera {
                                cuv__.replace_range(i..=i, &litera.to_string());
                                exista = true;
                            }

                            if cuv_joc == cuv__ {
                                println!("AI CASTIGAT. CUVANTUL A FOST {} SI AI AVUT {} INCERCARI GRESITE!", cuv__, incercari_gresite);

                                let mut mesaj = cuv_joc.to_string() + "\n";

                                if sectiune_sport {
                                    let mut file = OpenOptions::new()
                                        .write(true)
                                        .append(true)
                                        .open("src/Sport.txt")?;
                                    file.write_all(mesaj.as_bytes())?;
                                } else if sectiune_geografie {
                                    let mut file = OpenOptions::new()
                                        .write(true)
                                        .append(true)
                                        .open("src/Geografie.txt")?;
                                    file.write_all(mesaj.as_bytes())?;
                                } else if sectiune_gramatica {
                                    let mut file = OpenOptions::new()
                                        .write(true)
                                        .append(true)
                                        .open("src/Gramatica.txt")?;
                                    file.write_all(mesaj.as_bytes())?;
                                } else if sectiune_matematica {
                                    let mut file = OpenOptions::new()
                                        .write(true)
                                        .append(true)
                                        .open("src/Matematica.txt")?;
                                    file.write_all(mesaj.as_bytes())?;
                                }

                                mesaj.clear();
                                mesaj = cuv_joc.to_string()
                                    + "   "
                                    + &((cuv_final.len() - incercari_gresite) * 100).to_string()
                                    + " puncte\n";

                                let mut file = OpenOptions::new()
                                    .write(true)
                                    .append(true)
                                    .open("src/Scor.txt")?;
                                file.write_all(mesaj.as_bytes())?;

                                castigat = 1;
                                break;
                            }
                        }

                        if !exista {
                            incercari_gresite += 1;
                        } else {
                            exista = false;
                        }

                        if incercari_gresite == (cuv_joc.len() / 2 + 2) {
                            println!(
                                "AI PIERDUT. CUVANTUL A FOST {} SI NU AI REUSTI SA-L GHICESTI!",
                                cuv_joc
                            );
                            let mut mesaj = cuv_joc.to_string() + "\n";

                            if sectiune_sport {
                                let mut file = OpenOptions::new()
                                    .write(true)
                                    .append(true)
                                    .open("src/Sport.txt")?;
                                file.write_all(mesaj.as_bytes())?;
                            } else if sectiune_geografie {
                                let mut file = OpenOptions::new()
                                    .write(true)
                                    .append(true)
                                    .open("src/Geografie.txt")?;
                                file.write_all(mesaj.as_bytes())?;
                            } else if sectiune_gramatica {
                                let mut file = OpenOptions::new()
                                    .write(true)
                                    .append(true)
                                    .open("src/Gramatica.txt")?;
                                file.write_all(mesaj.as_bytes())?;
                            } else if sectiune_matematica {
                                let mut file = OpenOptions::new()
                                    .write(true)
                                    .append(true)
                                    .open("src/Matematica.txt")?;
                                file.write_all(mesaj.as_bytes())?;
                            }

                            mesaj.clear();
                            mesaj = cuv_joc.to_string()
                                + "   "
                                + &((cuv_final.len() - incercari_gresite) * 100).to_string()
                                + " puncte\n";

                            let mut file = OpenOptions::new()
                                .write(true)
                                .append(true)
                                .create(true)
                                .open("src/Scor.txt")?;
                            file.write_all(mesaj.as_bytes())?;
                        }

                        litera.clear();
                    }

                    if castigat == 1 {
                        break;
                    }
                }
                cuv_dictionar.clear();
                incepe_joc = 0;
            }
        }
    }
    Ok(())
}
