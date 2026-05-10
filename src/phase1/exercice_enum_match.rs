pub fn main() {
    // 🟢 Niveau 1 — Enums basiques
    // Exercice 1 — Enum simple
    // Définis un enum StatutTransaction avec quatre variantes : EnAttente, Confirmee, Echouee, Annulee. Dans main(), déclare une variable de chaque variante et affiche-la avec {:?}.
    // Exercice 2 — Match basique
    // Reprends StatutTransaction. Écris une fonction afficher_statut(statut: StatutTransaction) qui utilise un match pour afficher :

    // EnAttente → "Transaction en attente de confirmation..."
    // Confirmee → "Transaction confirmée ! ✅"
    // Echouee → "Transaction échouée ❌"
    // Annulee → "Transaction annulée"

    // Teste avec les quatre variantes.

    // 🟡 Niveau 2 — Enums avec données
    // Exercice 3 — Enum avec valeurs
    // Définis un enum Reseau avec trois variantes qui portent des données :

    // Bitcoin(f64) — le montant en BTC
    // Ethereum(f64) — le montant en ETH
    // Solana(f64) — le montant en SOL

    // Écris une fonction afficher_reseau(reseau: Reseau) qui affiche :

    // "Envoi de 0.5 BTC sur Bitcoin"
    // "Envoi de 2.3 ETH sur Ethereum"
    // "Envoi de 150.0 SOL sur Solana"

    // Exercice 4 — Enum avec struct interne
    // Définis un enum Transaction avec deux variantes :

    // Envoi { expediteur: String, destinataire: String, montant: f64 }
    // Reception { source: String, montant: f64 }

    // Écris une fonction traiter(tx: Transaction) qui affiche les détails selon la variante. Teste les deux cas.
    // Exercice 5 — Match avec condition (guard)
    // Reprends Reseau de l'exercice 3. Écris une fonction verifier_montant(reseau: Reseau) qui utilise des match guards (if dans le match) pour afficher :

    // Montant == 0.0 → "Montant invalide"
    // Montant < 0.01 → "Montant trop faible"
    // Montant >= 0.01 → "Montant valide : X"

    // 🟠 Niveau 3 — Enums et méthodes
    // Exercice 6 — impl sur un Enum
    // Définis un enum NoeudBitcoin avec trois variantes : Complet, Leger, Mineur. Implémente :

    // fn description(&self) -> &str — retourne une description de chaque type de nœud
    // fn est_mineur(&self) -> bool — retourne true seulement pour Mineur

    // Teste dans main().
    // Exercice 7 — Enum imbriqué
    // Définis un enum EvenementBlockchain avec les variantes :

    // NouveauBloc(u32) — l'index du bloc
    // NouvelleTransaction(Transaction) — réutilise l'enum Transaction de l'exercice 4
    // NoeudDeconnecte(String) — l'adresse IP du nœud

    // Écris une fonction traiter_evenement(evt: EvenementBlockchain) qui affiche les détails de chaque événement.

    // 🔴 Niveau 4 — Option, Result et Match
    // Exercice 8 — Option et Match
    // Écris une fonction trouver_bloc(blockchain: &Vec<String>, index: usize) -> Option<&String> qui retourne le bloc à l'index donné ou None si l'index est invalide. Dans main(), gère les deux cas avec match et avec if let.
    // Exercice 9 — Result et Match
    // Écris une fonction parser_montant(montant: &str) -> Result<f64, String> qui :

    // Retourne Ok(valeur) si le parsing réussit et que le montant est positif
    // Retourne Err("Montant invalide") si le parsing échoue
    // Retourne Err("Montant négatif") si la valeur est négative

    // Teste avec "25000", "abc", et "-500".
    // Exercice 10 — Tout ensemble : Simulateur de transactions
    // Construis un mini simulateur :
    // enum StatutTransaction { EnAttente, Confirmee, Echouee }
    // enum TypeTransaction { Envoi(f64), Reception(f64) }
    // struct Transaction { id: u32, type_tx: TypeTransaction, statut: StatutTransaction }
    // Implémente sur Transaction :

    // new(id: u32, type_tx: TypeTransaction) -> Self — statut EnAttente par défaut
    // confirmer(&mut self) — passe le statut à Confirmee
    // echouer(&mut self) — passe le statut à Echouee
    // afficher(&self) — affiche tous les détails avec match sur type_tx et statut

    // Dans main() : crée 3 transactions, confirme la première, fais échouer la deuxième, affiche toutes les trois.
}
