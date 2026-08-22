use uuid::Uuid;

#[derive(Debug, Clone)]

pub struct DeviceIdentity {
    pub id: String,

    pub name: String,
}

impl DeviceIdentity {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),

            name: name.to_string(),
        }
    }
}
