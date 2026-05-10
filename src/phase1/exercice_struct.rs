#[derive(Debug)]
struct Bloc {
    index: u32,
    donnees: String,
    precedent_hash: String,
    hash: String,
}

struct Mineur {
    nom: String,
    puissance: u32,
}

struct Blockchain {
    blocs: Vec<Bloc>,
}

struct BlocMine {
    bloc: Bloc,
    mineur: Mineur,
}

struct Mempool {
    transactions: Vec<String>,
}

impl Mempool {
    fn new() -> Self {
        Self {
            transactions: vec![],
        }
    }

    fn ajouter(&mut self, tx: String) {
        self.transactions.push(tx);
    }

    fn taille(&self) -> usize {
        return self.transactions.len();
    }

    fn afficher(&self) {
        for (index, tx) in self.transactions.iter().enumerate() {
            println!("Trx #{index}: {tx}");
        }
    }

    fn prochaine_transaction(&self) -> Option<&String> {
        self.transactions.first()
    }
}

impl Bloc {
    fn new(index: u32, donnees: String) -> Self {
        Self {
            index,
            donnees,
            precedent_hash: String::from("0000"),
            hash: String::from("0000"),
        }
    }

    fn afficher(&self) {
        println!("--- Bloc #{} ---", self.index);
        println!("Données     : {}", self.donnees);
        println!("Hash préc.  : {}", self.precedent_hash);
        println!("Hash actuel : {}", self.hash);
    }

    fn mettre_a_jour_hash(&mut self, nouveau_hash: String) {
        self.hash = nouveau_hash
    }

    fn genesis() -> Self {
        Self {
            index: 0,
            donnees: String::from("Genesis Block"),
            precedent_hash: String::from("0000000000000000"),
            hash: String::from("0000000000000000"),
        }
    }
}

impl Blockchain {
    fn new() -> Self {
        Self {
            blocs: vec![Bloc::genesis()],
        }
    }

    fn ajouter_bloc(&mut self, donnees: String) {
        let bloc = Bloc {
            index: (self.blocs.len() + 1) as u32,
            precedent_hash: String::from("0001"),
            hash: String::from("hash"),
            donnees,
        };

        self.blocs.push(bloc);
    }

    fn hauteur(&self) -> usize {
        return self.blocs.len();
    }

    fn afficher(&self) {
        for bloc in &self.blocs {
            bloc.afficher();
        }
    }

    // La blockchain n'est pas vide
    // Le premier bloc a index == 0
    // Chaque bloc a un index supérieur au précédent
    fn est_valide(&self) -> bool {
        // Condition 1 : blockchain non vide
        if self.blocs.is_empty() {
            return false;
        }

        // Condition 2 : premier bloc a index == 0
        if self.blocs[0].index != 0 {
            return false;
        }

        // Condition 3 : chaque bloc a un index supérieur au précédent
        for i in 1..self.blocs.len() {
            if self.blocs[i].index <= self.blocs[i - 1].index {
                return false;
            }
        }

        true
    }
}

pub fn main() {
    //     🟢 Niveau 1 — Définir et instancier
    // Exercice 1 — Struct basique
    // Définis une struct Bloc avec quatre champs : index: u32, donnees: String, precedent_hash: String, hash: String. Instancie-la manuellement dans main() et affiche chaque champ.
    let bloc = Bloc {
        index: 1,
        donnees: String::from("data"),
        precedent_hash: String::from("precedent_hash"),
        hash: String::from("hash"),
    };

    println!(
        "index: {}\ndonnées: {}\nhash précédent: {}\nhash: {}",
        bloc.index, bloc.donnees, bloc.precedent_hash, bloc.hash
    );

    // Exercice 2 — Fonction associée
    // Ajoute une fonction associée new(index: u32, donnees: String) -> Self à Bloc qui crée un bloc avec precedent_hash et hash initialisés à "0000" par défaut. Instancie deux blocs avec Bloc::new() et affiche-les.
    let bloc1: Bloc = Bloc::new(1, String::from("data"));
    let bloc2: Bloc = Bloc::new(2, String::from("data"));

    println!("Bloc #1: {:?}", bloc1);
    println!("Bloc #2: {:?}", bloc2);

    // Exercice 3 — Struct avec méthode
    // Ajoute une méthode afficher(&self) à Bloc qui affiche proprement :
    // --- Bloc #0 ---
    // Données     : Genesis
    // Hash préc.  : 0000
    // Hash actuel : 0000
    bloc.afficher();

    // 🟡 Niveau 2 — Méthodes et mutabilité
    // Exercice 4 — Méthode qui modifie
    // Ajoute une méthode mettre_a_jour_hash(&mut self, nouveau_hash: String) qui met à jour le champ hash du bloc. Crée un bloc mutable, mets à jour son hash et affiche avant/après.

    let mut mutable_bloc = Bloc::new(8, String::from("Data"));

    println!("Hash avant: {}", mutable_bloc.hash);

    mutable_bloc.mettre_a_jour_hash(String::from("0001"));
    println!("Hash après: {}", mutable_bloc.hash);

    // Exercice 5 — Struct imbriquée
    // Crée une struct Mineur avec les champs nom: String et puissance: u32 (en TH/s). Crée une struct BlocMine qui contient un Bloc et un Mineur. Instancie un BlocMine et affiche le nom du mineur et les données du bloc.

    let bloc_mine = BlocMine {
        bloc: Bloc::new(1, String::from("data")),
        mineur: Mineur {
            nom: String::from("mineur"),
            puissance: 12000,
        },
    };

    println!(
        "Mineur: {}, données: {}",
        bloc_mine.mineur.nom, bloc_mine.bloc.donnees
    );

    // Exercice 6 — Fonction associée avec logique
    // Ajoute à Bloc une fonction associée genesis() -> Self qui crée le tout premier bloc de la blockchain :

    // index: 0
    // donnees: "Genesis Block"
    // precedent_hash: "0000000000000000"
    // hash: "0000000000000000"

    // Affiche-le avec la méthode afficher().
    let genesis = Bloc::genesis();
    genesis.afficher();

    // 🟠 Niveau 3 — Structs et collections
    // Exercice 7 — Vec de Structs
    // Crée une struct Mempool avec un champ transactions: Vec<String>. Implémente :

    // new() -> Self — mempool vide
    // ajouter(&mut self, tx: String) — ajoute une transaction
    // taille(&self) -> usize — retourne le nombre de transactions
    // afficher(&self) — affiche toutes les transactions

    // Teste dans main() avec 4 transactions.
    let mut mempool = Mempool::new();

    mempool.ajouter(String::from("00001"));
    mempool.ajouter(String::from("00002"));
    mempool.ajouter(String::from("00003"));
    mempool.ajouter(String::from("00004"));

    mempool.afficher();

    // Exercice 8 — Méthode qui retourne une valeur
    // Ajoute à Mempool une méthode prochaine_transaction(&self) -> Option<&String> qui retourne la première transaction sans la retirer. Gère le cas Some et None avec un match.

    match mempool.prochaine_transaction() {
        Some(tx) => println!("Prochaine transaction : {}", tx),
        None => println!("Mempool vide"),
    }

    // 🔴 Niveau 4 — Tout ensemble
    // Exercice 9 — Blockchain complète
    // Crée une struct Blockchain avec blocs: Vec<Bloc>. Implémente :

    // new() -> Self — démarre avec le bloc genesis
    // ajouter_bloc(&mut self, donnees: String) — crée un nouveau Bloc avec l'index suivant et l'ajoute
    // hauteur(&self) -> usize — retourne le nombre de blocs
    // afficher(&self) — affiche tous les blocs avec Bloc::afficher()

    // Dans main(), crée une blockchain, ajoute 3 blocs, affiche la hauteur et tous les blocs.
    let mut blockchain = Blockchain::new();

    blockchain.ajouter_bloc(String::from("données1"));
    blockchain.ajouter_bloc(String::from("données2"));
    blockchain.ajouter_bloc(String::from("données3"));

    // Exercice 10 — Validation
    // Ajoute à Blockchain une méthode est_valide(&self) -> bool qui vérifie que :

    // La blockchain n'est pas vide
    // Le premier bloc a index == 0
    // Chaque bloc a un index supérieur au précédent

    // Teste avec une blockchain valide et affiche le résultat. 🦀
    blockchain.est_valide();
}
