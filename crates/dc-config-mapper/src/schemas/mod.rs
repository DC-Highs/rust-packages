//! Strict input schemas (`*In`, port of the zod schemas) and their normalized outputs (`*Out`).
//!
//! Every `*In` type rejects unknown top-level fields (like `z.object().strict()`), so a game update
//! that adds a field makes the mapper fail loudly instead of silently dropping data.

pub mod chests;
pub mod fog;
pub mod grid;
pub mod heroic;
pub mod items;
pub mod maze;
pub mod runner;
pub mod skills;
pub mod tower;
