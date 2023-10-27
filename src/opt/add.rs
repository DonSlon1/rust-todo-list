use clap::Args;
use crate::opt::todo::{Todo, Status};

#[derive(Args, Debug)]
pub struct Add {
    pub name: String,
    pub description: String,
    pub status: Status
}



impl Add {
    pub fn run (self)  {
        Todo::add(self.name, self.description, self.status);
    }
}
