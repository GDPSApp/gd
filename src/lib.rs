// #![forbid(unsafe_code)]

pub mod auth;
pub mod client;
pub mod crypto;
pub mod entities;
pub mod internals;
pub mod primitives;
pub mod thunk;

mod cow;

pub use client::{Authenticated, AuthenticatedState, Client, Simple, SimpleState};
pub use entities::{Artist, Song, SongReference};
pub use primitives::id::Id;
