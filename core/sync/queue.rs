use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct SyncQueue {

    pub pending:Vec<String>,

}

impl SyncQueue {

pub fn new()->Self {

Self{
pending:vec![]
}

}

}
