use marccheck::make_raw_records;
use marccheck::parse_raw_record;
use marccheck::*;

fn main() {
    // let raw_records = make_raw_records("./test-data/test_10.mrc");
    let raw_records: Vec<Vec<char>> = make_raw_records("./bench-data/Books.All.2016.part01.utf8");
    println!("Found {} raw_records. Parsing...", raw_records.len());
    let mut poorly_dated_records = vec![];
    for raw_record in raw_records {
        let parsed_record: Record = parse_raw_record(raw_record.to_vec());

        let pub_year_008 = get_year_from_008(&parsed_record);
        let pub_year_260 = get_year_from_a_data_field(&parsed_record, "260");
        let pub_year_264 = get_year_from_a_data_field(&parsed_record, "264");

        match (pub_year_008, pub_year_260, pub_year_264) {
            (None, None, None) => (),
            (Some(a), Some(b), _) => {
                if two_years_not_the_same(&a, &b) {
                    println!("{} != {}", a, b);
                    poorly_dated_records.push(parsed_record);
                }
            }
            _ => (),
        }
    }
    println!("Found {} raw_records.", raw_records.len());
    println!("Found {} poorly dated records!", poorly_dated_records.len());
}

fn two_years_not_the_same(a: &str, b: &str) -> bool {
    !clean_year(&b).contains(&clean_year(&a))
        && !&a.contains("?")
        && !&b.contains("?")
        && clean_year(&a).len() > 0
        && clean_year(&b).len() > 0
}

fn clean_year(year: &str) -> String {
    // clean_year.replace(".", "").replace("c", "")
    year.chars().filter(|c| c.is_ascii_digit()).collect()
}

fn get_year_estimate(year: &str) -> String {
    year.to_string()
        .replace(".", "")
        .replace("c", "")
        .replace("[", "")
        .replace("]", "")[0..4]
        .to_string()
}
