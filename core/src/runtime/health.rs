pub struct HealthReport {
    pub healthy: bool,
    pub encrypted: bool,
    pub locked: bool,
    pub message: String,
}

impl HealthReport {

    pub fn ok() -> Self {
        Self {
            healthy: true,
            encrypted: true,
            locked: true,
            message: "ORVPASS RUNTIME HEALTHY".into(),
        }
    }
}
