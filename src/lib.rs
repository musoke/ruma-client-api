//! Crate ruma_client_api contains serializable types for the requests and responses for each
//! endpoint in the [Matrix](https://matrix.org/) client API specification. These types can be
//! shared by client and server code.

#![feature(rustc_macro)]

extern crate ruma_identifiers;
extern crate serde;
#[macro_use] extern crate serde_derive;

/// Endpoints for the r0.x.x versions of the client API specification.
pub mod r0 {
    pub mod account;
    pub mod alias;
    pub mod config;
    pub mod contact;
    pub mod context;
    pub mod directory;
    pub mod filter;
    pub mod media;
    pub mod membership;
    pub mod presence;
    pub mod profile;
    pub mod push;
    pub mod receipt;
    pub mod redact;
    pub mod room;
    pub mod search;
    pub mod send;
    pub mod server;
    pub mod session;
    pub mod sync;
    pub mod tag;
    pub mod typing;
    pub mod voip;
}

/// GET /_matrix/client/versions
pub mod version;