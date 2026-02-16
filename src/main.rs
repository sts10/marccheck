use clap::Parser;
use marccheck::find_records_with_mismatched_pub_years;
use marccheck::make_raw_records;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// MARC file to check for record with mismatched publication dates. Should be in Utf-8 format (not XML)
    #[clap(name = "MARC FILES")]
    marc_file: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    // let raw_records: Vec<Vec<char>> = make_raw_records("./bench-data/Books.All.2016.part01.utf8");
    let raw_records: Vec<Vec<char>> = make_raw_records(cli.marc_file);
    let raw_records_found = raw_records.len();
    println!("Found {} raw_records. Parsing...", raw_records_found);
    let poorly_dated_records = find_records_with_mismatched_pub_years(raw_records);
    println!(
        "Found {} records with mismatched publication years",
        poorly_dated_records.len()
    );
}
