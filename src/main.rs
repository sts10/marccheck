use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    // let raw_records = make_raw_records("./test-data/test_10.mrc");
    let raw_records = make_raw_records("./bench-data/Books.All.2016.part01.utf8");
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

// The Directory immediately follows the Leader at the beginning
// of the record and is located at character position 24.
// Each Directory entry is 12 character positions in length and
// contains three portions: the field tag, the field length,
// and the starting character position.

// 00-02 - Tag
// Three ASCII numeric or ASCII alphabetic characters (upper case or lower case, but not both) that identify an associated variable field.
// 03-06 - Field length
// Four ASCII numeric characters that specify the length of the variable field, including indicators, subfield codes, data, and the field terminator. A Field length number of less than four digits is right justified and unused positions contain zeros.
// 07-11 - Starting character position
// Five ASCII numeric characters that specify the starting character position of the variable field relative to the Base address of data (Leader/12-16) of the record. A Starting character position number of less than five digits is right justified and unused positions contain zeros.

#[derive(Debug, Clone)]
struct Record {
    data: Vec<char>,
    fields: Vec<Field>,
    leader: Vec<char>, // It'd be neat to have this set to 24 characters?  &'a [char; 24],
}

#[derive(Debug, Clone)]
struct Field {
    tag: String,              // both Control and Data fields
    indicator1: Option<char>, // for Data fields. Maybe single char? Always digit?
    indicator2: Option<char>, // for Data fields. Maybe single char? Always digit?
    value: Option<String>,    // for Control fields
    // We use a HashMap for sub_fields for look-up efficiency, where char is the code and String is
    // the value
    // HashMap<char, &[char]> might be even more efficient, but maybe for
    // a subsequent pass.
    sub_fields: Option<HashMap<char, String>>, // for Data fields
}

fn parse_raw_record(raw_record: Vec<char>) -> Record {
    let mut fields: Vec<Field> = vec![];

    // this is awfu but it's the most sure way of finding the character langth of
    // the raw directory, which we need to know the starting position offset!
    let mut directory_size = 0;
    for ch in &raw_record[24..raw_record.len()] {
        if *ch == 0x1e as char {
            break;
        } else {
            directory_size = directory_size + 1;
        }
    }

    let starting_character_position_offset = directory_size + 24;
    let leader: &Vec<char> = &raw_record[0..24].to_vec(); // inefficient?
    assert!(leader.len() == 24);

    let mut print_start_of_next_record = false;

    for raw_directory_entry in raw_record[24..raw_record.len()].chunks_exact(12) {
        if raw_directory_entry.contains(&(0x1e as char)) {
            // we reached end of DIRECTORY
            break;
        }
        assert_eq!(number_cleaner(&['4', '1', '0', '0']), 4100);
        let field_length: usize = number_cleaner(&raw_directory_entry[3..=6]);

        let starting_character_position: usize = number_cleaner(&raw_directory_entry[7..=11]);
        let actual_starting_character_position =
            starting_character_position + starting_character_position_offset;

        let raw_field: String = chop_record_using_chars(
            &raw_record,
            actual_starting_character_position,
            field_length,
        );

        let tag: String = raw_directory_entry[0..=2].iter().collect();

        let value: Option<String> = if tag.starts_with("00") {
            Some(raw_field.to_string())
        } else {
            None
        };

        let sub_fields = if !tag.starts_with("00") {
            let subfields_as_raw_vec = split_and_vectorize(&raw_field, 0x1F as char);
            let mut temp_sub_fields = HashMap::new();
            for subfield_raw in subfields_as_raw_vec {
                if subfield_raw.len() > 2 {
                    temp_sub_fields.insert(
                        subfield_raw.chars().nth(0).unwrap(), // code
                        subfield_raw.chars().collect::<Vec<_>>()[1..]
                            .iter()
                            .collect::<String>(), // value
                    );
                }
            }
            Some(temp_sub_fields)
        } else {
            None
        };

        // Control fields do not have indicators!!
        let indicator1 = if !tag.starts_with("00") {
            Some(&raw_field.chars().nth(0).unwrap())
        } else {
            None
        };
        let indicator2 = if !tag.starts_with("00") {
            Some(
                &raw_field
                    .chars()
                    .nth(1)
                    .expect("Value too short to have an second indicator"),
            )
        } else {
            None
        };

        let this_field = Field {
            tag,
            value,                           // for Control fields only
            indicator1: indicator1.copied(), // Data fields only
            indicator2: indicator2.copied(), // Data fields only
            sub_fields,                      // for Data fields only
        };
        fields.push(this_field);
    }

    let this_record: Record = Record {
        leader: leader.to_vec(),
        data: raw_record.clone(), // I'm tired!
        fields: fields.clone(),
    };
    this_record
}

/// Delimiting on `0x1d as char`, chunk out individual records
/// as Vector of characters
fn make_raw_records(file_name: &str) -> Vec<Vec<char>> {
    // let chars = read_string_from_file_to_vector("./my-data/test_10.mrc").unwrap();
    let chars = read_string_from_file_to_vector(file_name).unwrap();
    let mut records: Vec<Vec<char>> = vec![vec![]]; // not sure if this initalization is corrent.
    let mut idx = 0;
    for ch in chars {
        records[idx].push(ch);
        if ch == 0x1d as char {
            // println!("Found end of a record!");
            idx = idx + 1;
            records.push(vec![]);
        }
    }
    // my poor code requires us to trim off the last record, which is
    // an empty Vector
    records.pop();
    records
}

/// Attempts to measure field_length in bytes, I think?
fn chop_record_using_chars(
    raw_record: &[char],
    starting_position: usize,
    field_length: usize,
) -> String {
    let mut raw_field = "".to_string();
    let mut char_index = 0;
    let mut byte_index = 0;
    for ch in raw_record {
        let mut buffer = [0; 4]; // can probably be 2 for utf8
        let ch_length_in_bytes = ch.encode_utf8(&mut buffer).len();
        char_index += 1;
        byte_index += ch_length_in_bytes;
        if byte_index >= starting_position && byte_index <= starting_position + field_length {
            raw_field.push(*ch);
        }
    }
    // raw_field.iter().collect::<String>()
    raw_field
}

/// Reads a text file into a Vector of `char`s (characters)
pub fn read_string_from_file_to_vector(file_path: &str) -> io::Result<Vec<char>> {
    let mut f = File::open(file_path.trim_matches(|c| c == '\'' || c == ' '))?;
    let mut string_from_file = String::new();
    f.read_to_string(&mut string_from_file)
        .expect("something went wrong reading the file");

    // println!("String: {}", string_from_file);
    let mut vector_of_chars = Vec::new();
    for c in string_from_file.chars() {
        // print!("{}", c);
        vector_of_chars.push(c);
    }
    Ok(vector_of_chars)
}

/// ```
/// assert_eq!(number_cleaner(&['0', '0', '1', '1']), 11);
/// assert_eq!(number_cleaner(&['0', '0', '0', '5', '4']), 54);
/// assert_eq!(number_cleaner(&['0', '0', '1', '0', '7']), 107);
/// assert_eq!(number_cleaner(&['4', '1', '0', '0']), 4100);
/// ```
fn number_cleaner(chs: &[char]) -> usize {
    let as_string: String = chs.iter().collect();
    as_string.parse().unwrap()
}

pub fn split_and_vectorize<'a>(string_to_split: &'a str, splitter: char) -> Vec<&'a str> {
    string_to_split.split(splitter).collect()
}
