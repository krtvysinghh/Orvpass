

pub fn fuzzy(
query:&str,
items:&Vec<String>
)->Vec<String>{


items

.iter()

.filter(
|x| x.to_lowercase()
.contains(&query.to_lowercase())
)

.cloned()

.collect()


}


