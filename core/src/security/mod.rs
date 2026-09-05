pub mod check;
pub mod final_check;
pub mod fingerprint;
pub mod integrity;
pub mod memory;
pub mod policy;
pub mod readiness;
pub mod release;

pub fn verify_mac_constant_time(expected: &[u8], actual: &[u8]) -> bool {
    crate::crypto::constant_time_eq(expected, actual)
}
pub mod lockout;

pub fn verify_kdf_parameters_minimums(memory_kib: u32, iterations: u32) -> bool {
    memory_kib >= 64 * 1024 && iterations >= 3
}
