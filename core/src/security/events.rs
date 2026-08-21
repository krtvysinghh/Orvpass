use super::audit;

pub fn vault_created() {
    audit::log("vault_created");
}

pub fn unlock_success() {
    audit::log("unlock_success");
}

pub fn unlock_failed() {
    audit::log("unlock_failed");
}

pub fn item_deleted() {
    audit::log("item_deleted");
}
