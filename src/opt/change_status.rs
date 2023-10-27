use clap::Args;
use crate::opt::todo::{Todo, Status};
#[derive(Args, Debug)]
pub struct ChangeStatus {
    pub id: i32,

     /// Options are: Completed, InProgress, NotStarted
    pub status: Status
}


impl ChangeStatus {
    pub fn run(self){
       Todo::change_status(self.id, self.status);
    }
}

