pub fn search(
query: &str,
items: Vec<String>
) -> Vec<String> {

items
.into_iter()
.filter(|x| x.to_lowercase().contains(&query.to_lowercase()))
.collect()

}
