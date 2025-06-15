//! Stories in Rust
//!
//! Main question is how to embed TSOB in rust
//! embedding this representation which cna represent anything
//!   in a PL that can represent anything
//! want minimal impact to downstream representations
//!   1 | that is, min diff in pre-compilation story/semantics
//!     w/e you can express in Rust can do so in TSOB Rust interface
//!       almost as concisely (can never be as concise ofc but 
//!         in some problems could actually be more if models domain more accurately)
//!   2 | and similar runtime semantics
//!     so need to make sure min overhead in definitions
//!       should run as fast as normal Rust program
//!
//! this is perhaps an open problem in general
//!   of how to embed a repr in another optimially
//!   also is subjective because of (1) depends on UX
//!
//! solution is to make a compiler
//!   so don't need to make story repr perfect
//!     can translate direct repr into Rust repr
//!   does this show anything in general?
//!     e.g. if stories are meant ot describe struct of thought
//!       mapping thought to rust is using min conceptual model within Rust lang
//!       like if surface area of lang interface was minified it 
//!         should look like code output from this compiler??
//!

pub mod being;
pub mod life;
pub mod control;


