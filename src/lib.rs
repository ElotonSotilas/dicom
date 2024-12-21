// Core module always included
pub mod core;

#[cfg(any(
    feature = "image", 
    feature = "default"
))]
pub mod image;

#[cfg(any(
    all(feature = "serde", feature = "compress", feature = "images"),
    feature = "default"
))]
pub mod interop;

#[cfg(any(
    all(feature = "fs", feature = "serde", feature = "compress", feature = "images"),
    feature = "default"
))]
pub mod mods;

#[cfg(any(
    feature = "net", 
    feature = "default"
))]
pub mod net;

#[cfg(any(
    all(feature = "net", feature = "serde", feature = "compress"),
    feature = "default"
))]
pub mod plugins;

#[cfg(any(
    feature = "cli",
    feature = "default"
))]
pub mod tools;

#[cfg(any(
    all(feature = "cli", feature = "compress", feature = "log", feature = "fs"),
    feature = "default"
))]
pub mod utils;

#[cfg(any(
    all(feature = "cli", feature = "compress", feature = "test", feature = "fs", feature = "serde"),
    feature = "default"
))]
pub mod validation;

#[cfg(any(
    all(feature = "net", feature = "secure"),
    feature = "default"
))]
pub mod web;

pub use core::*;
