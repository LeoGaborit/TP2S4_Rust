use clap::{Parser, Subcommand};
use std::collections::{HashMap, HashSet};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Voter {
        nom : String,
        choix: Option<String>,
    },
    Votants,
    Scores,
}

fn main() {
    let cli = Cli::parse();

    let mut votants : HashSet<String> = HashSet::new();
    let mut scores : HashMap<String, i32> = HashMap::new();

    match &cli.command {
        Commands::Voter { nom, choix } => {
            if votants.contains(nom) {
                println!("{} à déjà côté !", nom)
            }
            else {
                votants.insert(nom.clone());
                for candidat in scores.keys() {
                    if choix.equals(candidat) {
                        candidat //TODO : Finir l'ajout du score si candidat existe OU ajouter candidat
                    }
                }
                println!("{} vote pour {}", nom, choix)
            }
        }
        Commands::Votants => {
            if votants.is_empty() {
                println!("Il n'y a aucun votant !")
            } else {
                println!("Liste des votants : {:?}", votants)
            }
        }
        Commands::Scores => {
            if scores.is_empty() {
                println!("Aucun score n'a été enregistré !");
            } else {
                for (candidat, score) in scores {
                    println!("{} à obtenu {} voix", candidat, score)
                }
            }
        }
    }
}