# dfir-esedb


This crate provides a parser for Microsofts EseDB files, aimed to be used
for forensic purposes.

## Usage Example

```rust
use std::path::PathBuf;
use dfir_esedb::EseDb;
let db = EseDb::open(&PathBuf::from("tests/data/ntds_plain.dit")).unwrap();

assert_eq!(db.header().database_time().hours(), &21);
assert_eq!(db.header().database_time().minutes(), &45);
assert_eq!(db.header().database_time().seconds(), &2);
```
