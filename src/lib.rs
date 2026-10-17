//! `stats` — Handl stdlib port (`src/index.hnl`).
//!
//! Pure handl: the crate only re-exports the source so a consumer can embed
//! it until the `pkg.hnl` loader can fetch it by name.

pub const STATS_HNL: &str = include_str!("index.hnl");
