
use super::*;


pub struct BitwardenImporter;


impl Importer for BitwardenImporter {


fn import(
_data:&str
)->Result<Vec<ImportedItem>,String>{

Ok(Vec::new())

}

}

