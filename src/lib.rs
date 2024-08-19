//!
//! This crate provides a parser for Microsofts EseDB files, aimed to be used
//! for forensic purposes.
//! 
//! # Usage Example
//! 
//! ```rust
//! use std::path::PathBuf;
//! use dfir_esedb::EseDb;
//! let db = EseDb::open(&PathBuf::from("tests/data/ntds_plain.dit")).unwrap();
//! 
//! assert_eq!(db.header().database_time().hours(), &21);
//! assert_eq!(db.header().database_time().minutes(), &45);
//! assert_eq!(db.header().database_time().seconds(), &2);
//! ```

mod esedb;
mod error;
mod file_header;
mod file_format_version;
mod file_format_revision;
mod file_type;
mod db_time;
mod jet_signature;
mod jet_logtime;
mod jet_lgpos;
mod jet_bkinfo;
mod database_state;
mod page;
mod page_header;
mod page_flags;
mod page_tag;

pub use esedb::*;
pub use error::*;
pub use file_header::*;
pub use file_format_version::*;
pub use file_format_revision::*;
pub use file_type::*;
pub use db_time::*;
pub use jet_signature::*;
pub use jet_logtime::*;
pub use jet_lgpos::*;
pub use jet_bkinfo::*;
pub use database_state::*;
pub use page::*;
pub use page_header::*;
pub use page_flags::*;
pub use page_tag::*;

pub (crate) mod util;