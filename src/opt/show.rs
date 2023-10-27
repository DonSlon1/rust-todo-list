use clap::Args;
use crate::opt::todo::Todo;

#[derive(Args, Debug)]
pub struct Show {
    #[clap(short, long, action = clap::ArgAction::SetTrue, default_value = "false")]
    pub show_completed: Option<bool>
}



impl Show {
    pub fn run (self)  {
        Todo::show(self.show_completed);
    }
}
