#[derive(Debug)]
enum StatutTransaction {
    EnAttente,
    Confirmee,
    Echouee,
    Annulee,
}

enum Reseau {
    Bitcoin(f64),
    Ethereum(f64),
    Solana(f64),
}

enum Transaction {
    Envoi {
        expediteur: String,
        destinataire: String,
        montant: f64,
    },
    Reception {
        source: String,
        montant: f64,
    },
}

enum NoeudBitcoin {
    Complet,
    Leger,
    Mineur,
}

enum EvenementBlockchain {
    NouveauBloc(u32),
    NouvelleTransaction(Transaction),
    NoeudDeconnecte(String),
}

impl NoeudBitcoin {
    fn description(&self) -> &str {
        match self {
            NoeudBitcoin::Complet => "Nœud complet — vérifie toutes les transactions",
            NoeudBitcoin::Leger => "Nœud léger — télécharge uniquement les en-têtes",
            NoeudBitcoin::Mineur => "Nœud mineur — crée de nouveaux blocs",
        }
    }

    fn est_mineur(&self) -> bool {
        match self {
            NoeudBitcoin::Mineur => true,
            _ => false,
        }
    }
}

fn traiter_evenement(evt: EvenementBlockchain) {
    match evt {
        EvenementBlockchain::NouveauBloc(index) => {
            println!("Nouveau bloc miné ! Index : #{}", index)
        }
        EvenementBlockchain::NouvelleTransaction(tx) => {
            traiter(tx); // réutilise la fonction de l'exercice 4
        }
        EvenementBlockchain::NoeudDeconnecte(ip) => {
            println!("Nœud déconnecté : {}", ip)
        }
    }
}

fn traiter(tx: Transaction) {
    match tx {
        Transaction::Envoi {
            expediteur,
            destinataire,
            montant,
        } => {
            println!(
                "Envoi de {} BTC de {} vers {}",
                montant, expediteur, destinataire
            );
        }
        Transaction::Reception { source, montant } => {
            println!("Réception de {} BTC depuis {}", montant, source);
        }
    }
}

fn afficher_reseau(reseau: Reseau) {
    match reseau {
        Reseau::Bitcoin(montant) => println!("Envoi de {} BTC sur Bitcoin", montant),
        Reseau::Ethereum(montant) => println!("Envoi de {} ETH sur Ethereum", montant),
        Reseau::Solana(montant) => println!("Envoi de {} SOL sur Solana", montant),
        _ => (),
    }
}

fn afficher_statut(statut: StatutTransaction) {
    match statut {
        StatutTransaction::Confirmee => println!("Transaction confirmée ! ✅"),
        StatutTransaction::EnAttente => println!("Transaction en attente de confirmation..."),
        StatutTransaction::Annulee => println!("Transaction annulée"),
        StatutTransaction::Echouee => println!("Transaction échouée ❌"),
    }
}

fn verifier_montant(reseau: Reseau) {
    match reseau {
        Reseau::Bitcoin(m) | Reseau::Ethereum(m) | Reseau::Solana(m) if m == 0.0 => {
            println!("Montant invalide")
        }
        Reseau::Bitcoin(m) | Reseau::Ethereum(m) | Reseau::Solana(m) if m < 0.01 => {
            println!("Montant trop faible")
        }
        Reseau::Bitcoin(m) | Reseau::Ethereum(m) | Reseau::Solana(m) => {
            println!("Montant valide : {}", m)
        }
    }
}

fn trouver_bloc<'a>(blockchain: &'a Vec<String>, index: usize) -> Option<&'a String> {
    blockchain.get(index) // .get() retourne déjà Option<&T> ✅
}

fn parser_montant(montant: &str) -> Result<f64, String> {
    match montant.parse::<f64>() {
        Err(_) => Err(String::from("Montant invalide")),
        Ok(v) if v < 0.0 => Err(String::from("Montant négatif")),
        Ok(v) => Ok(v),
    }
}

enum TypeTransaction {
    Envoi(f64),
    Reception(f64),
}

struct SimTransaction {
    id: u32,
    type_tx: TypeTransaction,
    statut: StatutTransaction,
}

impl SimTransaction {
    fn new(id: u32, type_tx: TypeTransaction) -> Self {
        Self {
            id,
            type_tx,
            statut: StatutTransaction::EnAttente,
        }
    }

    fn confirmer(&mut self) {
        self.statut = StatutTransaction::Confirmee;
    }

    fn echouer(&mut self) {
        self.statut = StatutTransaction::Echouee;
    }

    fn afficher(&self) {
        let type_str = match &self.type_tx {
            TypeTransaction::Envoi(m) => format!("Envoi de {} BTC", m),
            TypeTransaction::Reception(m) => format!("Réception de {} BTC", m),
        };
        let statut_str = match &self.statut {
            StatutTransaction::EnAttente => "En attente",
            StatutTransaction::Confirmee => "Confirmée ✅",
            StatutTransaction::Echouee => "Échouée ❌",
            StatutTransaction::Annulee => "Annulée",
        };
        println!("TX #{} | {} | {}", self.id, type_str, statut_str);
    }
}

pub fn main() {
    // 🟢 Niveau 1 — Enums basiques
    // Exercice 1 — Enum simple
    // Définis un enum StatutTransaction avec quatre variantes : EnAttente, Confirmee, Echouee, Annulee. Dans main(), déclare une variable de chaque variante et affiche-la avec {:?}.
    let pending: StatutTransaction = StatutTransaction::EnAttente;
    let confirmed: StatutTransaction = StatutTransaction::Confirmee;
    let failed: StatutTransaction = StatutTransaction::Echouee;
    let cancelled: StatutTransaction = StatutTransaction::Annulee;

    println!("{:?}", pending);
    println!("{:?}", confirmed);
    println!("{:?}", failed);
    println!("{:?}", cancelled);
    println!("\n");

    // Exercice 2 — Match basique
    // Reprends StatutTransaction. Écris une fonction afficher_statut(statut: StatutTransaction) qui utilise un match pour afficher :

    // EnAttente → "Transaction en attente de confirmation..."
    // Confirmee → "Transaction confirmée ! ✅"
    // Echouee → "Transaction échouée ❌"
    // Annulee → "Transaction annulée"

    // Teste avec les quatre variantes.
    afficher_statut(pending);
    afficher_statut(confirmed);
    afficher_statut(failed);
    afficher_statut(cancelled);
    println!("\n");

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

    let btc: Reseau = Reseau::Bitcoin(0.5);
    let eth: Reseau = Reseau::Ethereum(2.3);
    let sol: Reseau = Reseau::Solana(150.0);

    afficher_reseau(btc);
    afficher_reseau(eth);
    afficher_reseau(sol);
    println!("\n");

    // Exercice 4 — Enum avec struct interne
    // Définis un enum Transaction avec deux variantes :

    // Envoi { expediteur: String, destinataire: String, montant: f64 }
    // Reception { source: String, montant: f64 }

    // Écris une fonction traiter(tx: Transaction) qui affiche les détails selon la variante. Teste les deux cas.
    let envoi: Transaction = Transaction::Envoi {
        expediteur: String::from("Alice"),
        destinataire: String::from("Bob"),
        montant: 1.3,
    };

    let recep: Transaction = Transaction::Reception {
        source: String::from("Alice"),
        montant: 1.2,
    };

    traiter(envoi);
    traiter(recep);

    // Exercice 5 — Match avec condition (guard)
    // Reprends Reseau de l'exercice 3. Écris une fonction verifier_montant(reseau: Reseau) qui utilise des match guards (if dans le match) pour afficher :

    // Montant == 0.0 → "Montant invalide"
    // Montant < 0.01 → "Montant trop faible"
    // Montant >= 0.01 → "Montant valide : X"

    let reseau: Reseau = Reseau::Bitcoin(1.3);
    afficher_reseau(reseau);

    // 🟠 Niveau 3 — Enums et méthodes
    // Exercice 6 — impl sur un Enum
    // Définis un enum NoeudBitcoin avec trois variantes : Complet, Leger, Mineur. Implémente :

    // fn description(&self) -> &str — retourne une description de chaque type de nœud
    // fn est_mineur(&self) -> bool — retourne true seulement pour Mineur

    // Teste dans main().

    let noeud_btc: NoeudBitcoin = NoeudBitcoin::Complet;
    noeud_btc.description();

    println!("Est mineur ? {}", noeud_btc.est_mineur());

    // Exercice 7 — Enum imbriqué
    // Définis un enum EvenementBlockchain avec les variantes :

    // NouveauBloc(u32) — l'index du bloc
    // NouvelleTransaction(Transaction) — réutilise l'enum Transaction de l'exercice 4
    // NoeudDeconnecte(String) — l'adresse IP du nœud

    // Écris une fonction traiter_evenement(evt: EvenementBlockchain) qui affiche les détails de chaque événement.

    let evt = EvenementBlockchain::NoeudDeconnecte(String::from("127.0.0.1"));
    traiter_evenement(evt);

    // 🔴 Niveau 4 — Option, Result et Match
    // Exercice 8 — Option et Match
    // Écris une fonction trouver_bloc(blockchain: &Vec<String>, index: usize) -> Option<&String> qui retourne le bloc à l'index donné ou None si l'index est invalide. Dans main(), gère les deux cas avec match et avec if let.

    let blockchain = vec![
        String::from("Bloc #0"),
        String::from("Bloc #1"),
        String::from("Bloc #2"),
    ];

    // Avec match
    match trouver_bloc(&blockchain, 1) {
        Some(bloc) => println!("Trouvé : {}", bloc),
        None => println!("Index invalide"),
    }

    // Avec if let — plus concis quand tu n'as besoin que du cas Some
    if let Some(bloc) = trouver_bloc(&blockchain, 5) {
        println!("Trouvé : {}", bloc);
    } else {
        println!("Index invalide");
    }

    // Exercice 9 — Result et Match
    // Écris une fonction parser_montant(montant: &str) -> Result<f64, String> qui :

    // Retourne Ok(valeur) si le parsing réussit et que le montant est positif
    // Retourne Err("Montant invalide") si le parsing échoue
    // Retourne Err("Montant négatif") si la valeur est négative

    // Teste avec "25000", "abc", et "-500".

    let cas = vec!["25000", "abc", "-500"];

    for c in cas {
        match parser_montant(c) {
            Ok(v) => println!("{} → Ok : {}", c, v),
            Err(e) => println!("{} → Err : {}", c, e),
        }
    }

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

    let mut tx1 = SimTransaction::new(1, TypeTransaction::Envoi(0.5));
    let mut tx2 = SimTransaction::new(2, TypeTransaction::Reception(1.2));
    let tx3 = SimTransaction::new(3, TypeTransaction::Envoi(0.1));

    tx1.confirmer();
    tx2.echouer();

    tx1.afficher();
    tx2.afficher();
    tx3.afficher();
}
