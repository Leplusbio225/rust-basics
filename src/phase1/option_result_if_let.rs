fn premier_element(liste: &[i32]) -> Option<i32> {
    liste.first().copied()
}

fn premier_element2(liste: &[i32]) {
    if let Some(v) = liste.first() {
        println!("Premier: {}", v);
    } else {
        println!("Liste vide");
    }
}

fn convertir_age(age: &str) -> Result<u8, String> {
    let number = match age.parse::<u8>() {
        Ok(number) if (0..150).contains(&number) => Ok(number),
        Err(_e) => Err(String::from("Pas un nombre")),
        Ok(_num) => Err("Age impossible".to_string()),
    };

    number
}

fn connect_node(ip: &str, port: &str) -> Result<String, String> {
    if ip.is_empty() {
        return Err(String::from("IP vide"));
    }

    match port.parse::<u16>() {
        Ok(p) => Ok(format!("Connecté à {ip}:{p}")),
        Err(_) => Err(String::from("Port invalide")),
    }
}

fn classify_transaction(montant: f64) {
    match montant {
        m if m < 0.0 => println!("Transaction invalide"),
        m if m == 0.0 => println!("Transaction nulle"),
        m if m < 100.0 => println!("Petite transaction"),
        m if m < 10_000.0 => println!("Transaction moyenne"),
        _ => println!("Grande transaction"),
    }
}

fn analyze_block(index: u32, taille: usize) {
    match (index, taille) {
        (i, _) if i == 0 => println!("Genesis Block"),
        (_, t) if t == 0 => println!("Empty Block"),
        (_, t) if t > 1_000 => println!("Fully Block"),
        _ => println!("Normal Block"),
    }
}

fn valid_address(address: &str) -> Result<&str, String> {
    if address.is_empty() || (address.len() < 10) {
        return Err(String::from("Invalid address"));
    }

    Ok(address)
}

fn valid_amount(amount: f64) -> Result<f64, String> {
    if amount <= 0.0 {
        return Err(String::from("Invalid amount"));
    }

    Ok(amount)
}

fn valid_network(network: &str) -> Result<&str, String> {
   match network {
        "bitcoin" | "solana" => Ok(network),
        _ => Err(String::from("Réseau inconnu")),
    }
}

fn create_transaction(address: &str, amount: f64, network: &str) -> Result<String, String> {
    let adresse = valid_address(address)?; // ? propage l'erreur automatiquement
    let montant = valid_amount(amount)?;
    let reseau = valid_network(network)?;

    Ok(format!(
        "Transaction créée : {} BTC vers {} sur {}",
        montant, adresse, reseau
    ))
}

pub fn main() {
    //     🟢 Niveau 1 — Option et Match
    // Exercice 1 — Option basique
    // Écris une fonction premier_element(liste: &[i32]) -> Option<i32> qui retourne le premier élément d'un slice ou None si vide. Dans main(), teste avec une liste non vide et une liste vide — gère les deux cas avec match.

    let numbers = [3, 0, 4];

    match premier_element(&numbers) {
        Some(v) => println!("Premier : {}", v),
        None => println!("Liste vide"),
    }

    match premier_element(&[]) {
        Some(v) => println!("Premier : {}", v),
        None => println!("Liste vide"),
    }

    // Exercice 2 — if let
    // Reprends premier_element. Cette fois gère le résultat avec if let au lieu de match. Affiche le message "Liste vide" si None.
    premier_element2(&numbers);
    premier_element2(&[]);

    // 🟡 Niveau 2 — Result
    // Exercice 3 — Result basique
    // Écris une fonction convertir_age(age: &str) -> Result<u8, String> qui :

    // Retourne Ok(valeur) si le parsing réussit et que l'âge est entre 0 et 150
    // Retourne Err("Pas un nombre") si le parsing échoue
    // Retourne Err("Age impossible") si la valeur est hors de 0-150

    // Teste avec "25", "abc", et "200".
    let r1 = convertir_age("25");
    let r2 = convertir_age("abc");
    let r3 = convertir_age("200");

    println!("{:?}", r1);
    println!("{:?}", r2);
    println!("{:?}", r3);

    // Exercice 4 — Chaîner les Results
    // Écris une fonction connecter_noeud(ip: &str, port: &str) -> Result<String, String> qui :

    // Parse le port avec port.parse::<u16>() — retourne Err("Port invalide") si échec
    // Vérifie que l'ip n'est pas vide — retourne Err("IP vide") si vide
    // Retourne Ok("Connecté à IP:PORT") si tout est valide

    // Teste avec ("192.168.1.1", "8333"), ("", "8333"), et ("192.168.1.1", "abc").
    let test1 = connect_node("192.168.1.1", "8333");
    let test2 = connect_node("", "8333");
    let test3 = connect_node("192.168.1.1", "abc");

    println!("Test1: {:?}", test1);
    println!("Test2: {:?}", test2);
    println!("Test3: {:?}", test3);

    // 🟠 Niveau 3 — Match guards

    // Exercice 5 — Guards sur nombres
    // Écris une fonction classifier_transaction(montant: f64) qui utilise des match guards pour afficher :

    // montant < 0.0 → "Transaction invalide"
    // montant == 0.0 → "Transaction nulle"
    // montant < 100.0 → "Petite transaction"
    // montant < 10000.0 → "Transaction moyenne"
    // montant >= 10000.0 → "Grande transaction"
    // Teste avec -5.0, 0.0, 50.0, 5000.0, 25000.0.

    classify_transaction(-5.0);
    classify_transaction(0.0);
    classify_transaction(50.0);
    classify_transaction(5000.0);
    classify_transaction(25000.0);

    // Exercice 6 — Guards sur tuples
    // Écris une fonction analyser_bloc(index: u32, taille: usize) qui utilise des match guards sur un tuple (index, taille) pour afficher :

    // index == 0 → "Bloc genesis"
    // taille == 0 → "Bloc vide"
    // taille > 1000 → "Bloc plein"
    // reste → "Bloc normal (#index)"

    // Teste avec (0, 500), (1, 0), (5, 2000), (3, 250).$
    analyze_block(0, 500);
    analyze_block(1, 0);
    analyze_block(5, 2000);
    analyze_block(3, 250);

    // 🔴 Niveau 4 — Tout ensemble
    // Exercice 7 — while let
    // Crée un Vec<Option<String>> contenant Some("TX1"), None, Some("TX2"), Some("TX3"), None. Utilise while let Some(item) = vecteur.pop() pour dépiler et afficher uniquement les Some — ignore les None avec if let.

    let mut vectors: Vec<Option<String>> = vec![
        Some(String::from("TX1")),
        None,
        Some(String::from("TX2")),
        Some(String::from("TX3")),
        None,
    ];

    while let Some(item) = vectors.pop() {
        if let Some(t) = item {
            println!("Transaction : {}", t);
        }
    }

    // Exercice 8 — Pipeline complet
    // Construis un pipeline de validation de transaction Bitcoin :
    // rustfn valider_adresse(adresse: &str) -> Result<&str, String>
    // fn valider_montant(montant: f64) -> Result<f64, String>
    // fn valider_reseau(reseau: &str) -> Result<&str, String>
    // fn creer_transaction(
    //     adresse: &str,
    //     montant: f64,
    //     reseau: &str
    // ) -> Result<String, String>

    // valider_adresse → Err si adresse vide ou longueur < 10
    // valider_montant → Err si montant <= 0.0
    // valider_reseau → Err si réseau n'est pas "bitcoin" ou "solana"
    // creer_transaction → appelle les trois validations, retourne Ok("Transaction créée : X BTC vers ADRESSE sur RESEAU") si tout passe

    // Dans main(), teste avec :

    // Une transaction valide
    // Une adresse trop courte
    // Un montant négatif
    // Un réseau inconnu

    // Gère chaque résultat avec match.

    let cas = [
        ("1A2B3C4D5E6F7G8H9I", 0.5, "bitcoin"),  // valide
        ("court", 0.5, "bitcoin"),               // adresse trop courte
        ("1A2B3C4D5E6F7G8H9I", -1.0, "bitcoin"), // montant négatif
        ("1A2B3C4D5E6F7G8H9I", 0.5, "dogecoin"), // réseau inconnu
    ];

    for (adresse, montant, reseau) in cas {
        match create_transaction(adresse, montant, reseau) {
            Ok(msg) => println!("{}", msg),
            Err(err) => println!("{}", err),
        }
    }
}
