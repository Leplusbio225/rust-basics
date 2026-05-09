pub fn slices_exercices() {
    // 🟢 Niveau 1 — String Slicing basique
    // Exercice 1 — Extraire des mots
    // Crée un String contenant "Solana Blockchain Rust". Extrais et affiche les trois mots séparément en utilisant des slices avec les bons indices.

    let sentence: String = String::from("Solana Blockchain Rust");

    let solana: &str = &sentence[0..6];
    let blockchain: &str = &sentence[7..17];
    let rust: &str = &sentence[18..];

    println!("Contenu de la variable solana {}", solana);
    println!("Contenu de la variable blockchain {}", blockchain);
    println!("Contenu de la variable rust {}", rust);

    // Exercice 2 — Raccourcis de slicing
    // Crée un String contenant "Abidjan Côte d'Ivoire". Affiche :

    // Tout ce qui va jusqu'à "Abidjan" avec [..n]
    // Tout ce qui commence après "Abidjan " avec [n..]
    // Le mot complet avec [..]

    let cote_divoire: String = String::from("Abidjan Côte d'Ivoire");
    println!("{}", &cote_divoire[..7]);
    println!("{}", &cote_divoire[8..]);
    println!("{}", &cote_divoire[..]);

    // 🟡 Niveau 2 — Array et Vec Slicing
    // Exercice 3 — Slice d'un array
    // Déclare un array de 6 hashrates [u32; 6]. Extrais :

    // Les 3 premiers avec &arr[..3]
    // Les 3 derniers avec &arr[3..]
    // Calcule la somme de chaque moitié et compare-les
    let hashrates: [u32; 6] = [12000, 36000, 42000, 56000, 62000, 75000];

    let three_first_part: &[u32] = &hashrates[..3];
    let three_last_part: &[u32] = &hashrates[3..];

    let sum_first_part: u32 = three_first_part.iter().sum();
    let sum_last_part: u32 = three_last_part.iter().sum();
    // let compare: bool = if sum_first_part > sum_last_part {
    //     true
    // } else {
    //     false
    // };

    let compare: bool = sum_first_part > sum_last_part;

    println!("3 First part {:?}", three_first_part);
    println!("3 last part {:?}", three_last_part);
    println!("sum first part {}", sum_first_part);
    println!("sum last part {}", sum_last_part);
    println!("Résultat de comparaison {}", compare);

    // Exercice 4 — Slice d'un Vec
    // Crée un Vec<String> de 5 noms de blockchains. Prends un slice des éléments du milieu (index 1 à 3) et affiche-les avec leur index.
    let blockchains: Vec<String> = vec![
        String::from("Bitcoin"),
        String::from("Ethereum"),
        String::from("Solana"),
        String::from("Dogecoin"),
        String::from("USDC"),
    ];

    let middle_btc: &[String] = &blockchains[1..4];

    for (index, btc) in middle_btc.iter().enumerate() {
        println!("Blockchain {}: {}", index + 1, btc);
    }

    let mot = premier_mot("Bitcoin est révolutionnaire");
    println!("Premier mot {}", mot); // "Bitcoin"

    // 🟠 Niveau 3 — Fonctions avec slices
    // Exercice 5 — Fonction qui prend un slice
    // Écris une fonction premier_mot(texte: &str) -> &str qui retourne le premier mot d'une phrase en trouvant le premier espace. Teste-la avec "Bitcoin est révolutionnaire".
    fn premier_mot(texte: &str) -> &str {
        let espace = texte.find(' ').unwrap_or(texte.len());

        &texte[..espace]
    }

    // Exercice 6 — Fonction générique sur slice
    // Écris une fonction afficher_slice(items: &[u32]) qui affiche chaque élément avec son index. Appelle-la avec un array complet, puis avec seulement une portion de cet array.

    let items: [u32; 5] = [1, 3, 4, 5, 10];
    afficher_slice(&items[1..]);

    fn afficher_slice(items: &[u32]) {
        for (index, item) in items.iter().enumerate() {
            println!("item {index}: {item}");
        }
    }

    // 🔴 Niveau 4 — Les pièges
    // Exercice 7 — Le piège UTF-8
    // Crée un String contenant "Côte d'Ivoire". Essaie d'abord de slicer avec des indices bytes pour extraire "Côte" — trouve les bons indices en comptant les bytes de 'ô' (2 bytes en UTF-8). Ensuite, fais la même chose proprement avec .chars().take(4).collect::<String>().

    let cote: String = String::from("Côte d'ivoire");

    // Méthode avec bytes
    let slicing_by_byte: &str = &cote[..5]; // 5 Car ô compte pour 2 bytes

    // Méthode avec chars().take()
    let clean_slicing: String = cote.chars().take(4).collect::<String>();

    println!("Slicing by bytes {}", slicing_by_byte);
    println!("Clean slicing {}", clean_slicing);
    // Exercice 8 — Blockchain mini avec slicing
    // Crée un Vec<String> de 6 blocs nommés "Bloc #1" à "Bloc #6". Écris une fonction afficher_plage(blocs: &[String], debut: usize, fin: usize) qui affiche uniquement les blocs entre les indices debut et fin. Appelle-la pour afficher :

    // Les blocs 2 à 4
    // Les 3 derniers blocs
    let blocs: [String; 6] = [
        String::from("Bloc #1"),
        String::from("Bloc #2"),
        String::from("Bloc #3"),
        String::from("Bloc #4"),
        String::from("Bloc #5"),
        String::from("Bloc #6"),
    ];

    // Utilisation correcte
    afficher_plage(&blocs, 1, 4); // blocs 2 à 4 (index 1, 2, 3)
    afficher_plage(&blocs, 3, 6); // 3 derniers (index 3, 4, 5)

    // Encore plus élégant pour les 3 derniers
    afficher_plage(&blocs, blocs.len() - 3, blocs.len());

    fn afficher_plage(blocs: &[String], debut: usize, fin: usize) {
        let plage = &blocs[debut..fin];
        for (index, bloc) in plage.iter().enumerate() {
            println!("Bloc {index}: {bloc}");
        }
    }
}
