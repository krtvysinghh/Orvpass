pub struct HealthReport {
    pub encrypted: bool,
    pub locked: bool,
    pub version: &'static str,
}

impl HealthReport {
    pub fn ok() -> Self {
        Self {
            encrypted: true,
            locked: true,
            version: "1.0.0",
        }
    }
}
