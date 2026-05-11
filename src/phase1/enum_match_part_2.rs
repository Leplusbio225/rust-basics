use std::ops::Index;

pub fn main() {
    // Enums

    #[derive(Debug)]
    enum Couleur {
        Rouge,
        Vert,
        Bleu,
        Jaune,
        Noir,
    }

    #[derive(Debug)]
    enum Direction {
        Nord,
        Sud,
        Est,
        Ouest,
    }

    enum Forme {
        Cercle(f64),
        Rectangle(f64, f64),
        Triangle(f64, f64, f64),
    }

    enum Message {
        Connexion { ip: String, port: u16 },
        Deconnexion { ip: String },
        Donnees { payload: String, taille: usize },
        Erreur { code: u32, description: String },
    }

    #[derive(Debug)]
    enum Saison {
        Printemps,
        Ete,
        Automne,
        Hiver,
    }

    enum Piece {
        Un,
        Cinq,
        Dix,
        Vingtcinq,
    }

    enum Classe {
        Guerrier,
        Mage,
        Archer,
    }
    enum Action {
        Attaquer,
        Defendre,
        Fuir,
    }

    struct Personnage {
        nom: String,
        classe: Classe,
        points_de_vie: u32,
        attaque: u32,
    }

    // Implémentations

    impl Saison {
        fn suivante(&self) -> Saison {
            match self {
                Saison::Printemps => Saison::Ete,
                Saison::Ete => Saison::Automne,
                Saison::Automne => Saison::Hiver,
                Saison::Hiver => Saison::Printemps,
            }
        }

        fn temperature_moyenne(&self) -> i32 {
            match self {
                Saison::Ete => 25,
                Saison::Automne => 26,
                Saison::Hiver => 27,
                Saison::Printemps => 28,
            }
        }

        fn afficher(&self) {
            println!("{:?}: {:?}°", self.suivante(), self.temperature_moyenne())
        }
    }

    impl Piece {
        fn valeur(&self) -> u32 {
            const CENTIME: u32 = 100;

            match self {
                Piece::Un => 1,
                Piece::Cinq => 5,
                Piece::Dix => 10,
                Piece::Vingtcinq => 25,
            }
        }

        fn est_rare(&self) -> bool {
            match self {
                Piece::Un => false,
                Piece::Cinq => false,
                Piece::Dix => false,
                Piece::Vingtcinq => true,
            }
        }
    }

    impl Personnage {
        fn new(nom: &str, classe: Classe) -> Self {
            let (points_de_vie, attaque) = match classe {
                Classe::Guerrier => (100, 15),
                Classe::Mage => (60, 30),
                Classe::Archer => (80, 20),
            };
            Self {
                nom: String::from(nom),
                classe,
                points_de_vie,
                attaque,
            }
        }

        fn agir(&self, action: Action) {
            match (&self.classe, action) {
                (Classe::Guerrier, Action::Attaquer) => {
                    println!("{} charge avec son épée !", self.nom)
                }
                (Classe::Mage, Action::Attaquer) => {
                    println!("{} lance une boule de feu !", self.nom)
                }
                (Classe::Archer, Action::Attaquer) => println!("{} décoche une flèche !", self.nom),
                (_, Action::Defendre) => println!("{} se met en garde", self.nom),
                (_, Action::Fuir) => println!("{} prend ses jambes à son cou !", self.nom),
            }
        }

        fn afficher(&self) {
            let classe = match self.classe {
                Classe::Guerrier => "Guerrier",
                Classe::Mage => "Mage",
                Classe::Archer => "Archer",
            };
            println!(
                "{} | {} | {} pv | {} atk",
                self.nom, classe, self.points_de_vie, self.attaque
            );
        }
    }

    // Méthodes
    fn code_hex(couleur: Couleur) -> &'static str {
        match couleur {
            Couleur::Bleu => "0000FF",
            Couleur::Jaune => "FFFF00",
            Couleur::Noir => "000000",
            Couleur::Rouge => "FF0000",
            Couleur::Vert => "008000",
        }
    }

    fn opposee(direction: Direction) -> Direction {
        match direction {
            Direction::Est => Direction::Ouest,
            Direction::Nord => Direction::Sud,
            Direction::Ouest => Direction::Est,
            Direction::Sud => Direction::Nord,
        }
    }

    fn perimetre(forme: Forme) -> f64 {
        const PI: f64 = 3.14;

        match forme {
            Forme::Cercle(r) => 2.0 * PI * r,
            Forme::Rectangle(l1, l2) => (l1 + l2) * 2.0,
            Forme::Triangle(c1, c2, c3) => c1 + c2 + c3,
        }
    }

    fn logger(msg: Message) {
        match msg {
            Message::Connexion { ip, port } => {
                println!("Message de connexion: ip => {} et port => {}", ip, port)
            }
            Message::Deconnexion { ip } => println!("Deconnexion: ip => {ip}"),
            Message::Donnees { payload, taille } => {
                println!("Données: payload => {payload}, taille => {taille} ")
            }
            Message::Erreur { code, description } => {
                println!("Erreur: code => {code}, description => {description}")
            }
        }
    }

    fn classifier(forme: Forme) {
        match forme {
            Forme::Cercle(r) => {
                if r == 0.0 {
                    println!("Cercle invalide")
                } else if r >= 5.0 {
                    println!("Grand cercle")
                } else {
                    println!("Petit cercle")
                }
            }
            Forme::Rectangle(l1, l2) => {
                if l1 == l2 {
                    println!("Carré")
                } else {
                    println!("Rectangle")
                }
            }
            Forme::Triangle(c1, c2, c3) => println!("Triangle de périmètre P = {}", c1 + c2 + c3),
        }
    }

    fn total(pieces: &[Piece]) -> u32 {
        let mut total: u32 = 0;

        pieces.into_iter().for_each(|p: &Piece| {
            total += p.valeur();
        });

        total
    }

    fn chercher_ville(villes: &[String], nom: &str) -> Option<usize> {
        villes.iter().position(|x| x == nom)
    }

    fn diviser(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Division par zéro impossible"))
        } else {
            Ok(a / b)
        }
    }

    fn calculer(operations: &[(f64, f64)]) {
        for (a, b) in operations.iter() {
            match diviser(*a, *b) {
                Ok(resultat) => println!("{} / {} = {:.2}", a, b, resultat),
                Err(e) => println!("Erreur : {}", e),
            }
        }
    }

    // 🟢 Niveau 1 — Enums basiques
    // Exercice 1 — Enum de couleurs
    // Définis un enum Couleur avec cinq variantes : Rouge, Vert, Bleu, Jaune, Noir. Écris une fonction code_hex(couleur: Couleur) -> &'static str qui retourne le code hexadécimal de chaque couleur. Teste les cinq.

    const BLUE: Couleur = Couleur::Bleu;
    const RED: Couleur = Couleur::Rouge;
    const YELLOW: Couleur = Couleur::Jaune;
    const GREEN: Couleur = Couleur::Vert;
    const BLACK: Couleur = Couleur::Noir;

    let blue_hex: &str = code_hex(BLUE);
    let red_hex: &str = code_hex(RED);
    let yellow_hex: &str = code_hex(YELLOW);
    let green_hex: &str = code_hex(GREEN);
    let black_hex: &str = code_hex(BLACK);
    println!(
        "Bleu: {blue_hex}\nRouge: {red_hex}\nYellow: {yellow_hex}\nGreen: {green_hex}\nBlack: {black_hex}"
    );

    // Exercice 2 — Enum de directions
    // Définis un enum Direction avec quatre variantes : Nord, Sud, Est, Ouest. Écris une fonction opposee(direction: Direction) -> Direction qui retourne la direction opposée. Affiche le résultat pour les quatre directions.

    let nord: Direction = opposee(Direction::Nord);
    let sud: Direction = opposee(Direction::Sud);
    let est: Direction = opposee(Direction::Est);
    let ouest: Direction = opposee(Direction::Ouest);

    println!("----DIRECTION OPPOSEE-----");
    println!(
        "Nord: {:?}\nSud: {:?}\nEst: {:?}\nOuest: {:?}",
        nord, sud, est, ouest
    );

    // 🟡 Niveau 2 — Enums avec données
    // Exercice 3 — Formes géométriques
    // Définis un enum Forme avec trois variantes :

    // Cercle(f64) — le rayon
    // Rectangle(f64, f64) — largeur et hauteur
    // Triangle(f64, f64, f64) — trois côtés

    // Écris une fonction perimetre(forme: Forme) -> f64 qui calcule et retourne le périmètre de chaque forme. Teste les trois.
    let rec: f64 = perimetre(Forme::Rectangle(4.5, 6.5));
    let cer: f64 = perimetre(Forme::Cercle(2.5));
    let tri: f64 = perimetre(Forme::Triangle(2.5, 2.5, 2.5));

    println!("Rectangle: {rec}\nCercle: {cer}\nTriangle: {tri}");

    // Exercice 4 — Messages réseau
    // Définis un enum Message avec quatre variantes :

    // Connexion { ip: String, port: u16 }
    // Deconnexion { ip: String }
    // Donnees { payload: String, taille: usize }
    // Erreur { code: u32, description: String }

    // Écris une fonction logger(msg: Message) qui affiche les détails de chaque message. Teste les quatre variantes.

    logger(Message::Connexion {
        ip: String::from("127.0.0.1"),
        port: 3306,
    });
    logger(Message::Deconnexion {
        ip: String::from("127.0.0.1"),
    });
    logger(Message::Donnees {
        payload: String::from("payload"),
        taille: 2048,
    });
    logger(Message::Erreur {
        code: 404,
        description: String::from("Not Found"),
    });

    // Exercice 5 — Match guards
    // Reprends Forme de l'exercice 3. Écris une fonction classifier(forme: Forme) qui utilise des match guards pour afficher :

    // Cercle avec rayon == 0.0 → "Cercle invalide"
    // Cercle avec rayon < 5.0 → "Petit cercle"
    // Cercle avec rayon >= 5.0 → "Grand cercle"
    // Rectangle dont largeur == hauteur → "Carré"
    // Rectangle normal → "Rectangle"
    // Triangle → "Triangle"
    let cercle: Forme = Forme::Cercle(2.5);
    let rec: Forme = Forme::Rectangle(2.5, 2.5);
    let tri: Forme = Forme::Triangle(1.2, 2.4, 3.4);

    classifier(cercle);
    classifier(rec);
    classifier(tri);

    // 🟠 Niveau 3 — impl sur Enums
    // Exercice 6 — Saisons
    // Définis un enum Saison avec quatre variantes : Printemps, Ete, Automne, Hiver. Implémente :

    // fn suivante(&self) -> Saison — retourne la saison suivante
    // fn temperature_moyenne(&self) -> i32 — retourne une température moyenne réaliste pour Abidjan
    // fn afficher(&self) — affiche le nom et la température

    // Teste un cycle complet des quatre saisons.

    let printemps: Saison = Saison::Printemps;
    let ete: Saison = Saison::Ete;
    let automne: Saison = Saison::Automne;
    let hiver: Saison = Saison::Hiver;

    printemps.afficher();
    ete.afficher();
    automne.afficher();
    hiver.afficher();

    // Exercice 7 — Pièces de monnaie
    // Définis un enum Piece avec quatre variantes : Un, Cinq, Dix, Vingtcinq. Implémente :

    // fn valeur(&self) -> u32 — retourne la valeur en centimes
    // fn est_rare(&self) -> bool — retourne true pour Vingtcinq

    // Écris une fonction total(pieces: &[Piece]) -> u32 qui calcule la somme totale. Teste avec un slice de pièces mélangées.

    let piece_un: Piece = Piece::Un;
    let piece_cinq: Piece = Piece::Cinq;
    let piece_dix: Piece = Piece::Dix;
    let piece_vcq: Piece = Piece::Vingtcinq;

    println!("Cinq est rare ? {}", piece_cinq.est_rare());
    println!("Un est rare ? {}", piece_un.est_rare());
    println!("Dix est rare ? {}", piece_dix.est_rare());
    println!("Vingtcinq est rare ? {}", piece_vcq.est_rare());

    let total: u32 = total(&[piece_cinq, piece_un, piece_dix, piece_vcq]);

    println!("Valeur totale des pièces: {total}");

    // 🔴 Niveau 4 — Option, Result, if let
    // Exercice 8 — Chercher dans une liste
    // Crée un Vec<String> de 6 noms de villes ivoiriennes. Écris une fonction chercher_ville(villes: &[String], nom: &str) -> Option<usize> qui retourne l'index de la ville si elle existe. Dans main() :

    // Gère le résultat avec match
    // Gère le résultat avec if let
    // Teste avec une ville qui existe et une qui n'existe pas

    let villes: Vec<String> = vec![
        String::from("Abidjan"),
        String::from("Aboisso"),
        String::from("Yamoussokro"),
        String::from("Bouaké"),
        String::from("San-Pédro"),
        String::from("Dimbokro"),
    ];

    let index = chercher_ville(&villes, "Aboisso");

    println!("Aboisso est à la position {:?}", index.unwrap());

    // Exercice 9 — Calculatrice sécurisée
    // Écris une fonction diviser(a: f64, b: f64) -> Result<f64, String> qui :

    // Retourne Ok(résultat) si b != 0.0
    // Retourne Err("Division par zéro impossible") si b == 0.0

    // Écris ensuite une fonction calculer(operations: &[(f64, f64)]) qui itère sur une liste d'opérations, appelle diviser pour chacune, et affiche soit le résultat soit l'erreur avec match.

    let ops: &[(f64, f64)] = &[(10.0, 2.0), (5.0, 0.0), (9.0, 3.0)];
    calculer(ops);

    // 🔴 Exercice 10 — RPG Mini
    // Construis un mini système de jeu :
    // rustenum Classe { Guerrier, Mage, Archer }
    // enum Action { Attaquer, Defendre, Fuir }

    // struct Personnage {
    //     nom: String,
    //     classe: Classe,
    //     points_de_vie: u32,
    //     attaque: u32,
    // }
    // Implémente sur Personnage :

    // new(nom: &str, classe: Classe) -> Self — les stats varient selon la classe :

    // Guerrier → 100 pv, 15 attaque
    // Mage → 60 pv, 30 attaque
    // Archer → 80 pv, 20 attaque

    // agir(&self, action: Action) — affiche ce que fait le personnage selon son action ET sa classe :

    // Guerrier + Attaquer → "[Nom] charge avec son épée !"
    // Mage + Attaquer → "[Nom] lance une boule de feu !"
    // Archer + Attaquer → "[Nom] décoche une flèche !"
    // N'importe qui + Defendre → "[Nom] se met en garde"
    // N'importe qui + Fuir → "[Nom] prend ses jambes à son cou !"

    // afficher(&self) — affiche nom, classe, pv et attaque

    // Dans main() : crée un personnage de chaque classe, fais-les agir avec différentes actions, affiche leurs stats.

    // Dans main()
    let guerrier = Personnage::new("Kofi", Classe::Guerrier);
    let mage = Personnage::new("Aminata", Classe::Mage);
    let archer = Personnage::new("Jhon", Classe::Archer);

    guerrier.agir(Action::Attaquer);
    mage.agir(Action::Attaquer);
    archer.agir(Action::Attaquer);
    guerrier.agir(Action::Defendre);
    mage.agir(Action::Fuir);

    guerrier.afficher();
    mage.afficher();
    archer.afficher();
}
