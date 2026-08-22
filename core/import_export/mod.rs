pub mod formats;

pub trait Importer {
    fn name(&self) -> &'static str;
    fn import(&self, data: &str) -> Result<Vec<crate::models::Item>, String>;
}

pub trait Exporter {
    fn name(&self) -> &'static str;
    fn export(&self, items: &[crate::models::Item]) -> Result<String, String>;
}
