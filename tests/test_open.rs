use std::path::PathBuf;

use dfir_esedb::EseDb;

#[test]
fn test_open() {
    let db = EseDb::open(&get_test_data("ntds_plain.dit")).unwrap();
}

fn get_test_data(filename: &str) -> PathBuf {
    let mut data_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    data_path.push("tests");
    data_path.push("data");
    data_path.push(filename);

    data_path
}
