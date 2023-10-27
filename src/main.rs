use clap::{Parser, Subcommand};

mod opt;
mod db;

use crate::opt::change_status::ChangeStatus;
use crate::opt::add::Add;
use crate::opt::show::Show;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
   #[clap(subcommand)]
   pub entity_type: Option<Function>,

   #[clap(short, long, action = clap::ArgAction::SetTrue, default_value = "false")]
   pub show_completed: Option<bool>,

}

#[derive(Subcommand, Debug)]
pub enum Function {
   /// create
   Add(Add),
   /// delete
   ChangeStatus(ChangeStatus),
   /// Show
   Show(Show)
}



fn main() {
   let args = Cli::parse();
   match args.entity_type {
      None => {
         Show{show_completed: args.show_completed}.run();
         return;
      },
      Some(ent_type) => {
         match ent_type {
            Function::Add(x) => x.run(),
            Function::ChangeStatus(x) => x.run(),
            Function::Show(x) => x.run()
         }
      }
   }
}
