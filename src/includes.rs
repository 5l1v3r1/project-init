//! Read in files at compile time, making them available no matter what

// Licenses
pub const BSD3:&'static str = include_str!("includes/licenses/BSD3");
pub const BSD:&'static str = include_str!("includes/licenses/BSD");
pub const GPL3:&'static str = include_str!("includes/licenses/GPL3");
pub const MIT:&'static str = include_str!("includes/licenses/MIT");
pub const ALL_RIGHTS_RESERVED:&'static str = include_str!("includes/licenses/AllRightsReserved");

// README.md
pub const README:&'static str = include_str!("includes/README.md");

// Rust includes 
pub const CARGO_TOML:&'static str = include_str!("includes/rust/Cargo.toml.init");
pub const RUST_TEMPLATE:&'static str = include_str!("includes/rust/template.toml");
pub const RUST_LIB:&'static str = include_str!("includes/rust/lib.rs.init");
pub const RUST_TRAVIS_CI:&'static str = include_str!("includes/rust/.travis.yml");

// Vim includes
pub const VIMBALL:&'static str = include_str!("includes/vim/vimball.txt");
pub const VIM_TEMPLATE:&'static str = include_str!("includes/vim/template.toml");

// Python includes
pub const PY_BIN:&'static str = include_str!("includes/python/bin.py");
pub const PY_SETUP:&'static str = include_str!("includes/python/setup.py");
pub const PY_CFG:&'static str = include_str!("includes/python/setup.cfg");
pub const PY_TEMPLATE:&'static str = include_str!("includes/python/template.toml");
