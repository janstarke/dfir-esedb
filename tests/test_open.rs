use std::path::PathBuf;

use dfir_esedb::EseDb;

#[test]
fn test_open() {
    let db = EseDb::open(&get_test_data("ntds_plain.dit")).unwrap();

    assert_eq!(db.header().database_time().hours(), &21);
    assert_eq!(db.header().database_time().minutes(), &45);
    assert_eq!(db.header().database_time().seconds(), &2);
    assert_eq!(
        db.header()
            .database_signature()
            .creation_date_and_time()
            .years(),
        &2017
    );
    assert_eq!(
        db.header()
            .database_signature()
            .creation_date_and_time()
            .months(),
        &11
    );
    assert_eq!(
        db.header()
            .database_signature()
            .creation_date_and_time()
            .days(),
        &10
    );
    assert_eq!(
        db.header()
            .database_signature()
            .creation_date_and_time()
            .hours(),
        &19
    );
    assert_eq!(
        db.header()
            .database_signature()
            .creation_date_and_time()
            .minutes(),
        &25
    );
    assert_eq!(
        db.header()
            .database_signature()
            .creation_date_and_time()
            .seconds(),
        &43
    );
    assert_eq!(db.header().database_signature().computer_name(), "");
}

fn get_test_data(filename: &str) -> PathBuf {
    let mut data_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    data_path.push("tests");
    data_path.push("data");
    data_path.push(filename);

    data_path
}

#[test]
fn test_pages() {
    let db = EseDb::open(&get_test_data("ntds_plain.dit")).unwrap();

    assert_eq!(db.pages()[&0x2000].tags().len(), 13);
    assert_eq!(db.pages()[&0x4000].tags().len(), 1);
    assert_eq!(db.pages()[&0x6000].tags().len(), 9);
    assert_eq!(db.pages()[&0x8000].tags().len(), 4);
    assert_eq!(db.pages()[&0xA000].tags().len(), 18);
}
