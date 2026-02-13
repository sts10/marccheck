use marccheck::make_raw_records;
use marccheck::parse_raw_record;
use marccheck::*;

fn main() {
    // let raw_records = make_raw_records("./test-data/test_10.mrc");
    let raw_records: Vec<Vec<char>> = make_raw_records("./bench-data/Books.All.2016.part01.utf8");
    println!("Found {} raw_records.", raw_records.len());
    for raw_record in raw_records {
        let parsed_record: Record = parse_raw_record(raw_record.to_vec());

        let mut pub_year_008 = "".to_string();
        let mut pub_year_260_or_264 = "".to_string();
        for field in parsed_record.fields {
            if field.tag == "008" {
                // since we know this the is 008 field, we know it will have a value, so we can
                // safely unwrap here.
                let field_value = field.value.unwrap();
                // https://www.oclc.org/bibformats/en/fixedfield/dtst.html
                // https://www.oclc.org/bibformats/en/fixedfield/dates.html
                // https://www.oclc.org/bibformats/en/2xx/264.html
                pub_year_008 = if field_value.chars().nth(8).unwrap() == 's' {
                    (field_value[9..13]).to_string()
                } else {
                    (field_value[8..12]).to_string()
                };
            } else if field.tag == "260" || field.tag == "264" {
                // We know this is Data Field, since it's 260, so we can unwrap
                pub_year_260_or_264 = if field.sub_fields.clone().unwrap().contains_key(&'c') {
                    field.sub_fields.unwrap()[&'c'].clone()
                } else {
                    "".to_string() // we'll make this variable an Option later
                };
            }
        }
        if pub_year_008 != "" && pub_year_260_or_264 != "" && pub_year_008 != pub_year_260_or_264 {
            // Now check for weird edge case (string formatting?)
            if pub_year_008.parse::<usize>().is_ok()
                && pub_year_260_or_264.parse::<usize>().is_ok()
                && pub_year_008.parse::<usize>() == pub_year_260_or_264.parse::<usize>()
            {
                println!(
                    "Found messy record! ({} != {}). Leader is {}",
                    pub_year_008,
                    pub_year_260_or_264,
                    parsed_record.leader.iter().collect::<String>()
                );
            }
        } else {
            // println!("{} == {}", pub_year_008, pub_year_260_or_264);
        }
    }
}
