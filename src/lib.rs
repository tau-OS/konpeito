//! Konpeito Personal Key-value store
//! Konpeito is a local encrypted key-value database that can be used to store any type of data.
//! It is powered by sled and uses your SSH key by default.
//!
//! ## Usage
//! To use Konpeito as a library and retrieve key/values
//! ```rust
//! use konpeito::KeyStore;
//! ```

//export key for main
pub mod key;
pub use key::KeyStore;