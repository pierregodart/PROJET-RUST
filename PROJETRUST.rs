use std::io;
use std::collections::HashSet;

#[derive(Debug)]
struct Livre {
    titre: String,
    auteur: String,
    annee: u32,
    est_disponible: bool,
}

fn main() {
    let mut bibliotheque: Vec<Livre> = Vec::new();
    let mut titres_uniques = HashSet::new();

    println!(" Gestionnaire de bibliothèque v1.0");

    loop {
        afficher_menu();
        let choix = lire_entree_utilisateur();

        match choix.as_str() {
            "1" => ajouter_livre(&mut bibliotheque, &mut titres_uniques),
            "2" => gerer_emprunt(&mut bibliotheque, false),
            "3" => gerer_emprunt(&mut bibliotheque, true),
            "4" => afficher_livres(&bibliotheque, "Tous les livres"),
            "5" => afficher_livres(
                &bibliotheque
                    .iter()
                    .filter(|l| l.est_disponible)
                    .collect::<Vec<_>>(),
                "Livres disponibles",
            ),
            "6" => {
                println!("À bientôt !");
                break;
            }
            _ => println!(" Choix invalide"),
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
    io::Write::flush(&mut io::stdout()).unwrap();
}

fn lire_entree_utilisateur() -> String {
    let mut entree = String::new();
    io::stdin()
        .read_line(&mut entree)
        .expect("Erreur de lecture");
    entree.trim().to_string()
}

fn ajouter_livre(biblio: &mut Vec<Livre>, titres: &mut HashSet<String>) {
    println!("\n➕ Ajout d'un nouveau livre");
    
    let titre = demander_info("Titre");
    if !titres.insert(titre.clone()) {
        println!(" Ce livre existe déjà !");
        return;
    }

    biblio.push(Livre {
        titre,
        auteur: demander_info("Auteur"),
        annee: demander_info("Année").parse().unwrap_or(2023),
        est_disponible: true,
    });
    println!(" Livre ajouté !");
}

fn demander_info(champ: &str) -> String {
    println!("{} : ", champ);
    lire_entree_utilisateur()
}

fn gerer_emprunt(biblio: &mut Vec<Livre>, est_retour: bool) {
    let action = if est_retour { "retourner" } else { "emprunter" };
    println!("\n🔁 Livre à {} : ", action);
    let titre = lire_entree_utilisateur();

    match biblio.iter_mut().find(|l| l.titre == titre) {
        Some(livre) => {
            if livre.est_disponible == est_retour {
                println!(" Livre déjà {} !", if est_retour { "rendu" } else { "emprunté" });
            } else {
                livre.est_disponible = est_retour;
                println!("0ération réussie !");
            }
        }
        None => println!(" Livre introuvable"),
    }
}

fn afficher_livres(livres: &[&Livre], message: &str) {
    println!("\n📚 {} :", message);
    
    if livres.is_empty() {
        println!("Aucun livre à afficher");
        return;
    }

    for livre in livres {
        println!(
            "- {} par {} ({}) [{}]",
            livre.titre,
            livre.auteur,
            livre.annee,
            if livre.est_disponible { "Disponible" } else { "Emprunté" }
        );
    }
}