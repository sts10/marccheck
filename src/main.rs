use marccheck::find_records_with_mismatched_pub_years;
use marccheck::make_raw_records;
// use marccheck::*;

fn main() {
    // let raw_records = make_raw_records("./test-data/test_10.mrc");
    let raw_records: Vec<Vec<char>> = make_raw_records("./bench-data/Books.All.2016.part01.utf8");
    let raw_records_found = raw_records.len();
    println!("Found {} raw_records. Parsing...", raw_records_found);
    let poorly_dated_records = find_records_with_mismatched_pub_years(raw_records);
    // println!("Found {} raw_records.", raw_records_found);
    println!(
        "Found {} records with mismatched publication years!",
        poorly_dated_records.len()
    );
}
