use clap::{Parser, Subcommand};

use rage_flip_rust::clipboard::copy_to_clipboard;
use rage_flip_rust::chaos;
use rage_flip_rust::flipper;
use rage_flip_rust::frustrated;
use rage_flip_rust::sarcasm;
use rage_flip_rust::strikethrough;
use rage_flip_rust::text_substitution;
use rage_flip_rust::underline;

#[derive(Parser)]
#[command(name = "rage_flip")]
#[command(about = "A collection of text transformations")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Flip text upside down")]
    Flip {
        #[arg(required = false, default_value = "")]
        text: String,
    },
    #[command(about = "Rage flip with text")]
    RageFlip {
        #[arg(required = false, default_value = "")]
        text: String,
    },
    #[command(about = "Table flip with text")]
    TableFlip {
        #[arg(required = false, default_value = "")]
        text: String,
    },
    #[command(about = "Flip text upside down (alias for flip)")]
    FlipText {
        #[arg(required = false, default_value = "")]
        text: String,
    },
    #[command(about = "Apply sarcasm to text")]
    Sarcasm {
        #[arg(required = false, default_value = "")]
        text: String,
    },
    #[command(about = "Apply strikethrough to text")]
    Strikethrough {
        #[arg(required = false, default_value = "")]
        text: String,
    },
    #[command(about = "Apply underline to text")]
    Underline {
        #[arg(required = false, default_value = "")]
        text: String,
    },
    #[command(about = "Apply double underline to text")]
    Doubleunderline {
        #[arg(required = false, default_value = "")]
        text: String,
    },
    #[command(about = "Apply chaos to text")]
    Chaos {
        #[arg(required = false, default_value = "")]
        text: String,
    },
    #[command(about = "Set chaos level")]
    ChaosLevel {
        #[arg(required = true)]
        instruction: String,
    },
    #[command(about = "Apply text substitution")]
    Emote {
        #[arg(required = true)]
        substitution: String,
    },
    #[command(about = "Show disapproval emoji")]
    Disapproval {},
    #[command(about = "Apply frustrated ALL CAPS to text")]
    Frustrated {
        #[arg(required = false, default_value = "")]
        text: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let output = match cli.command {
        Commands::Flip { text } => flipper::flip(&text),
        Commands::RageFlip { text } => flipper::rage_flip(&text),
        Commands::TableFlip { text } => flipper::table_flip(&text),
        Commands::FlipText { text } => flipper::flip(&text),
        Commands::Sarcasm { text } => sarcasm::process(&text),
        Commands::Strikethrough { text } => strikethrough::process(&text),
        Commands::Underline { text } => underline::single_underline(&text),
        Commands::Doubleunderline { text } => underline::double_underline(&text),
        Commands::Chaos { text } => chaos::process(&text, None),
        Commands::ChaosLevel { instruction } => {
            match chaos::set_chaos_level(&instruction) {
                Ok(level) => {
                    println!("Chaos level set to: {}", level);
                    return;
                }
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Emote { substitution } => {
            if !text_substitution::substitution_exists(&substitution) {
                eprintln!("Error: '{}' is not a valid text substitution.", substitution);
                eprintln!("Use 'rage_flip emote --help' to see available substitutions.");
                std::process::exit(1);
            }
            text_substitution::process(&substitution).unwrap().to_string()
        }
        Commands::Disapproval {} => {
            text_substitution::process("disapproval").unwrap().to_string()
        }
        Commands::Frustrated { text } => frustrated::process(&text),
    };

    if output.is_empty() {
        return;
    }

    println!("{}", output);
    copy_to_clipboard(&output);
}
