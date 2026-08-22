use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Attachment {

pub name:String,
pub size:u64,
pub encrypted:bool,

}
