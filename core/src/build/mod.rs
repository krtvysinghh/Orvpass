pub const BUILD_CHANNEL: &str = "stable";
pub const BUILD_PROFILE: &str = "release";

pub fn identity() -> (&'static str, &'static str) {
    (BUILD_CHANNEL, BUILD_PROFILE)
}
