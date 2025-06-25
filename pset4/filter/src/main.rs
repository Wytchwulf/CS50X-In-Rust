use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Filter", about = "Image Filter Tool")]
struct Cli {
    #[command(subcommand)]
    command: FilterCommand,
}

#[derive(Subcommand)]
enum FilterCommand {
    #[command(visible_alias = "g")]
    Greyscale {
        input: String,
        output: String,
    },

    #[command(visible_alias = "s")]
    Sepia {
        input: String,
        output: String,
    },

    #[command(visible_alias = "r")]
    Reflect {
        input: String,
        output: String,
    },

    #[command(visible_alias = "b")]
    Blur {
        input: String,
        output: String,
    },

}


fn main() {
    let cli = Cli::parse();

    match cli.command {
        FilterCommand::Greyscale { input, output } => {
            println!("Applying greyscale: {} -> {}", input, output);
        }
        FilterCommand::Sepia { input, output } => {
            println!("Applying sepia: {} -> {}", input, output);
        }
        FilterCommand::Reflect { input, output } => {
            println!("Applying reflect: {} -> {}", input, output);
        }
        FilterCommand::Blur { input, output } => {
            println!("Applying blur: {} -> {}", input, output);
        }
    }
}