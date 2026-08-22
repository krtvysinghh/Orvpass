use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub version:String,
    pub entries:Vec<String>,
}

impl Database {

pub fn new()->Self {
Self{
version:"2.0".into(),
entries:vec![]
}
}

}
