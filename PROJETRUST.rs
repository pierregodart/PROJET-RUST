use std::collections::HashSet;
use std::io::{self, Write};

#[derive(Debug)]
struct Livre {
    titre: String,
    auteur: String,
    annee: u32,
    disponible: bool,
}

fn main() {
    let mut bibliotheque = Vec::new();
    let mut titres = HashSet::new();

    println!("Gestionnaire de bibliothèque v1.0");

    loop {
        afficher_menu();
        let choix = lire_entree();

        match choix.as_str() {
            "1" => ajouter_livre(&mut bibliotheque, &mut titres),
            "2" => changer_etat_livre(&mut bibliotheque, false),
            "3" => changer_etat_livre(&mut bibliotheque, true),
            "4" => lister_livres(&bibliotheque, "Tous les livres"),
            "5" => lister_livres(
                &bibliotheque.iter().filter(|l| l.disponible).collect::<Vec<_>>(),
                "Livres disponibles",
            ),
            "6" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide"),
        }
    }
}

fn afficher_menu() {
    println!("\n――――――――――――――――――――――");
    println!("1. Ajouter un livre");
    println!("2. Emprunter un livre");
    println!("3. Retourner un livre");
    println!("4. Lister tous les livres");
    println!("5. Lister les disponibles");
    println!("6. Quitter");
    print!("→ Votre choix : ");
    io::stdout().flush().unwrap();
}

fn lire_entree() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    input.trim().to_string()
}

fn ajouter_livre(biblio: &mut Vec<Livre>, titres: &mut HashSet<String>) {
    println!("\nAjout d'un nouveau livre");
    
    let titre = demander_champ("Titre");
    if !titres.insert(titre.clone()) {
        println!("Ce livre existe déjà !");
        return;
    }

    biblio.push(Livre {
        titre,
        auteur: demander_champ("Auteur"),
        annee: demander_champ("Année").parse().unwrap_or(2023),
        disponible: true,
    });
    println!("Livre ajouté !");
}

fn demander_champ(champ: &str) -> String {
    println!("{} : ", champ);
    lire_entree()
}

fn changer_etat_livre(biblio: &mut Vec<Livre>, rendre: bool) {
    let action = if rendre { "retourner" } else { "emprunter" };
    println!("\nLivre à {} : ", action);
    let titre = lire_entree();

    if let Some(livre) = biblio.iter_mut().find(|l| l.titre == titre) {
        if livre.disponible == rendre {
            let etat = if rendre { "rendu" } else { "emprunté" };
            println!("Livre déjà {} !", etat);
        } else {
            livre.disponible = rendre;
            println!("Opération réussie !");
        }
    } else {
        println!("Livre introuvable");
    }
}

fn lister_livres(livres: &[&Livre], message: &str) {
    println!("\n{} :", message);
    
    if livres.is_empty() {
        println!("Aucun livre");
        return;
    }

    for livre in livres {
        let etat = if livre.disponible { "Disponible" } else { "Emprunté" };
        println!("- {} par {} ({}) [{}]", 
                livre.titre, livre.auteur, livre.annee, etat);
    }
}
