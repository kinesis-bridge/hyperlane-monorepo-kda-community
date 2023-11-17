pub mod prelude {
    pub use more_asserts::*;

    const DEFAULT_PUBKEY_STR: &str = "83a5cfcdcbec1d513a2d02ab0f0b61e30c9be22d9c09af001affe79885414450";
    const DEFAULT_PRIVKEY_STR: &str = "79e3b0161be8d6e200cb1d3736625eb0daf20a12186687c72cdcfeab18845444";

    pub fn default_pubkey() -> String {
        DEFAULT_PUBKEY_STR.to_string()
    }

    pub fn default_privkey() -> String {
        DEFAULT_PRIVKEY_STR.to_string()
    }
}