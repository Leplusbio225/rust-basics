pub fn exercices() {
    level1();
    level2();
    level3();
    level4();
}

fn level1() {
    // LEVEL 1

    // Exercice 1 — Les types de base
    // Déclare cinq variables : ton âge en u8, ta taille en f32, ton prénom en &str, un booléen est_developpeur à true, et un caractère initiale. Affiche-les tous avec println!.

    let age: u8 = 40;
    let taille: f32 = 1.80;
    let prenom: &str = "Jhon Doe";
    let est_developpeur: bool = true;

    println!(
        "Age: {}, taille: {}, prénom: {}, est développeur ? {}",
        age, taille, prenom, est_developpeur
    );

    // Exercice 2 — Arithmétique typée
    // Déclare deux variables a: i32 = 150 et b: i32 = 47. Calcule et affiche leur somme, différence, produit, quotient et reste. Ensuite convertis a en f64 et divise-le par 3.0 — affiche le résultat.

    let a: i32 = 150;
    let b: i32 = 45;

    let somme: i32 = a + b;
    let difference = a - b;
    let produit = a * b;
    let quotient = a / b;
    let modulo = a % b;

    println!("Somme de {a} + {b} = {}", somme);
    println!("Différence de {a} - {b} = {}", difference);
    println!("Produit de {a} * {b} = {}", produit);
    println!("Quotient de {a} / {b} = {}", quotient);
    println!("Modulo de {a} % {b} = {}", modulo);

    let a: f64 = a as f64 / 3.0;

    println!("{a} / 3.0 en f64 = {}", a);
}

fn level2() {
    //  Exercice 3 — Tuple
    // Crée un tuple qui représente une transaction Bitcoin : (expediteur, destinataire, montant) avec les types (&str, &str, f64). Déstructure-le et affiche chaque champ séparément.

    let transaction_btc: (&str, &str, f64) = ("Alice", "Bob", 100.0);

    let (expediteur, destinataire, montant): (&str, &str, f64) = transaction_btc;

    println!("Expéditeur: {expediteur}, destinataire: {destinataire}, montant: {montant}");

    // Exercice 4 — Array
    // Déclare un tableau fixe de 5 hashrates de mineurs en u32. Calcule la somme totale et la moyenne. Affiche le premier et le dernier élément sans hardcoder les indices.

    let hashrates: [u32; 5] = [1200, 55000, 780000, 3200000, 45000000];

    let somme_hashrate: u32 = {
        let mut sum: u32 = 0;

        for hashrate in hashrates {
            sum = sum + hashrate;
        }

        sum
    };

    let first = hashrates.first();
    let last = hashrates.last();

    println!("La somme des hashrates est {somme_hashrate}");
    println!("First element {}", first.unwrap());
    println!("Last element {:?}", last.unwrap());

    // Exercice 5 — Vec
    // Crée un Vec<String> vide qui représente une liste de blocs. Ajoute 3 blocs avec .push(), affiche le nombre de blocs avec .len(), puis retire le dernier avec .pop() et affiche ce qui a été retiré.

    let mut block_lists: Vec<String> = Vec::new();

    block_lists.push(String::from("Block1"));
    block_lists.push(String::from("Block2"));
    block_lists.push(String::from("Block3"));

    let block_length = block_lists.len();

    let last_element: Option<String> = block_lists.pop();

    println!("Blocks length {block_length}");
    println!("Blocks last element {:?}", last_element);
}

fn level3() {
    //     Exercice 6 — Fonction simple
    // Écris une fonction est_pair(n: u32) -> bool qui retourne true si le nombre est pair. Teste-la dans main() avec plusieurs valeurs et affiche le résultat.
    est_pair(2);
    est_pair(3);
    est_pair(23);

    println!("7 est pair ? {}", est_pair(7));

    fn est_pair(n: u32) -> bool {
        if n % 2 == 0 { true } else { false }
        // n % 2 == 0
    }

    // Exercice 7 — Fonction avec plusieurs paramètres
    // Écris une fonction calculer_frais(montant: f64, taux: f64) -> f64 qui retourne les frais de transaction. Dans main(), appelle-la avec un montant de 50000.0 FCFA et un taux de 0.02. Affiche le montant final après frais.
    let montant = calculer_frais(50000.0, 0.02);

    println!("Montant final {}", montant);

    fn calculer_frais(montant: f64, taux: f64) -> f64 {
        montant + (montant * taux)
    }

    // Exercice 8 — Shadowing dans une fonction
    // Écris une fonction convertir_montant(montant: &str) -> f64 qui reçoit un montant en &str comme "25000", le convertit en f64 avec le shadowing, et retourne le résultat multiplié par 1.05 (frais inclus).
    let montant_shadowed = &convertir_montant("25000");

    println!("Montant shadowed {}", montant_shadowed);

    fn convertir_montant(montant: &str) -> f64 {
        let montant = montant.parse::<f64>().unwrap();

        montant * 1.05
    }
}

fn level4() {
    // 🔴 Niveau 4 — Ownership
    // Exercice 9 — Move vs Borrow
    // Crée un String appelé bloc_genesis. Passe-le à une fonction afficher_bloc(bloc: &String) qui l'affiche. Après l'appel, affiche bloc_genesis dans main() pour prouver qu'il existe toujours. Ensuite, écris une deuxième version consommer_bloc(bloc: String) qui prend ownership — et observe ce qui se passe si tu essaies d'utiliser bloc_genesis après.

    let block_genesis: String = String::from("Block Genesis");
    println!("{block_genesis}");

    consommer_bloc(block_genesis);
    // println!("{block_genesis}");

    fn consommer_bloc(bloc: String) {
        println!("{bloc}");
    }

    let mut blockchain = Blockchain::new();

    blockchain.add_block(String::from("block1"));
    blockchain.add_block(String::from("block2"));
    blockchain.add_block(String::from("block3"));
    blockchain.afficher();

    // Exercice 10 — Blockchain mini
    // Crée une struct Blockchain avec un Vec<String> pour les blocs. Implémente new() qui retourne une blockchain vide, add_block(&mut self, bloc: String) qui ajoute un bloc, et afficher(&self) qui affiche tous les blocs avec leur numéro. Dans main(), crée une blockchain, ajoute 3 blocs, et affiche-les tous.

    #[derive(Debug)]
    struct Blockchain {
        blocks: Vec<String>,
    }

    impl Blockchain {
        fn new() -> Blockchain {
            Blockchain { blocks: Vec::new() }
        }

        fn add_block(&mut self, bloc: String) {
            self.blocks.push(bloc);
        }

        fn afficher(&self) {
            for (index, b) in self.blocks.iter().enumerate() {
                println!("Bloc #{} : {}", index + 1, b);
            }
        }
    }
}
