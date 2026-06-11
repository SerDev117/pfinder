pub mod search;

use std::ptr::null;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "project-finder")]
#[command(version = "0.1")]
#[command(about = "this program is made for helping developers easily find their projects")]
struct Cli {
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
     find {
        name: Option<String>,

         #[arg(short, long, ignore_case = true)]
         language: Option<Language>,
    },
}
#[derive(ValueEnum, Clone, Copy, Debug)]
enum Language {
    rust,

    python,

    java,

    cpp,

    #[value(alias = "javascript")]
    js,


}
fn main() {
  let cli = Cli::parse();
        match &cli.command {
         Commands::find {name, language} => {
             let name_option = name.as_deref();
            match language {
                Some(lang) => {  search::find(*lang, name_option);}
                None => { println!("specify a language with --language | -l") }
            }


         }
    }

}
