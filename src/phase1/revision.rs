pub fn revision_globale() {
    // 🟢 Niveau 1 — Variables, Types, Constants
    // Exercice 1 — Variables et mutabilité
    // Déclare une constante MAX_BLOCS: u32 valant 1000000. Déclare une variable mutable blocs_mines qui commence à 0 et incrémente jusqu'à 5 dans une boucle for. Affiche à chaque itération : "Bloc #X miné — Total : Y/MAX_BLOCS".
    // Exercice 2 — Shadowing et types
    // Déclare une variable montant qui vaut "50000" en &str. Utilise le shadowing pour la convertir en f64, puis à nouveau en String formatée "50000 FCFA". Affiche chaque étape avec le type obtenu.
    // Exercice 3 — Tuple et destructuring
    // Crée un tuple transaction de type (&str, &str, f64, bool) représentant (expediteur, destinataire, montant, confirmee). Déstructure-le et affiche : "Transaction de Kofi vers Aminata : 25000 FCFA — Confirmée : true".

    // 🟡 Niveau 2 — Control Flow
    // Exercice 4 — if/else expressif
    // Écris une fonction statut_bloc(hauteur: u32) -> &'static str qui retourne :

    // "Genesis" si hauteur == 0
    // "Confirmé" si hauteur <= 6
    // "Mature" si hauteur > 6

    // Teste avec les hauteurs 0, 3, et 10.
    // Exercice 5 — Match
    // Déclare un enum Reseau avec trois variantes : Bitcoin, Ethereum, Solana. Écris un match qui affiche les frais moyens de chaque réseau :

    // Bitcoin → "Frais : ~5$"
    // Ethereum → "Frais : ~15$"
    // Solana → "Frais : ~0.001$"

    // Exercice 6 — Boucles
    // Écris une boucle loop qui simule le mining. À chaque itération, génère un nombre aléatoire entre 0 et 100 avec ce code :
    // rustlet nonce: u32 = (std::time::SystemTime::now()
    //     .duration_since(std::time::UNIX_EPOCH)
    //     .unwrap()
    //     .subsec_nanos()) % 100;
    // Si nonce < 10 → affiche "Bloc miné ! Nonce : X" et quitte la boucle. Sinon → affiche "Nonce {X} invalide, on continue...".

    // 🟠 Niveau 3 — Fonctions + Ownership
    // Exercice 7 — Fonction pure
    // Écris une fonction est_valide_hash(hash: &str) -> bool qui retourne true si le hash commence par "0000" (simulation d'un Proof of Work). Teste avec "0000abc123" et "abc0000123".
    // Exercice 8 — Ownership et move
    // Crée un String appelé adresse_wallet. Passe-le à une fonction verifier_adresse(adresse: String) -> bool qui retourne true si la longueur est supérieure à 10. Essaie d'utiliser adresse_wallet après l'appel — observe l'erreur. Corrige en passant &String à la place.
    // Exercice 9 — Borrowing mutable
    // Crée un Vec<String> appelé mempool (liste de transactions en attente). Écris une fonction ajouter_transaction(mempool: &mut Vec<String>, tx: String) qui ajoute une transaction. Écris une fonction afficher_mempool(mempool: &Vec<String>) qui affiche toutes les transactions. Appelle les deux dans main().

    // 🔴 Niveau 4 — Slices + Error Handling
    // Exercice 10 — Slicing avancé
    // Crée un Vec<String> de 8 transactions nommées "TX #1" à "TX #8". Écris une fonction dernier_bloc(transactions: &[String], taille: usize) -> &[String] qui retourne les taille dernières transactions. Affiche le résultat pour les 3 dernières.
    // Exercice 11 — Option
    // Écris une fonction trouver_transaction(mempool: &[String], id: &str) -> Option<&String> qui retourne la première transaction contenant id. Teste avec un id qui existe et un qui n'existe pas — gère les deux cas avec match.
    // Exercice 12 — Tout ensemble : Mini mempool
    // Construis une mini simulation de mempool Bitcoin :

    // Un Vec<String> de transactions
    // Une fonction ajouter_tx(mempool: &mut Vec<String>, tx: String)
    // Une fonction miner_bloc(mempool: &mut Vec<String>, taille_bloc: usize) -> Vec<String> qui retire et retourne les taille_bloc premières transactions
    // Une fonction afficher_etat(mempool: &[String]) qui affiche le nombre de transactions en attente et leur liste

    // Dans main() : ajoute 6 transactions, affiche l'état, mine un bloc de 4, affiche l'état final.

    println!("Révision")
}
