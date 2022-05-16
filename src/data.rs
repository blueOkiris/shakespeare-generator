/*
 * Author: Dylan Turner
 * Description: Convert data from CSV to representation to raw "bits". Pretty simple as just strings
 */

use csv::{
    Error, Reader
};
use serde::Deserialize;

const NAME_LEN: usize = 100; // Max name length
pub const NUM_INPUTS: usize = NAME_LEN * 8; // In bits
pub const NUM_OUTPUTS: usize = NAME_LEN * 8; // In bits

/* This is how we'll load in data from the CSV */

#[derive(Debug, Deserialize, Clone)]
pub struct TableEntry {
    pub initial_line: String,
    pub following_line: String
}

impl TableEntry {
    pub fn table_from_file(fname: &str) -> Result<Vec<Self>, Error> {
        let mut data = Vec::new();

        let mut reader = Reader::from_path(fname)?;
        for result in reader.deserialize() {
            let record: TableEntry = result?;
            data.push(record);
        }

        Ok(data)
    }
}

/*
 * And this is how we represent it internally
 * Note:
 * - Date: 00000 000.0 000000 0 - Day Month Year Left over bit
 */

pub fn from_table_entry(entry: &TableEntry) -> (Vec<u8>, Vec<u8>) {
    let initial_line = format!("{: >100}", entry.initial_line.to_ascii_lowercase())
        .as_bytes()[0..NAME_LEN].try_into()
        .expect("Failed to parse initial line!");
    let final_line = format!("{: >100}", entry.following_line.to_ascii_lowercase())
        .as_bytes()[0..NAME_LEN].try_into()
        .expect("Failed to parse following line!");

    (initial_line, final_line)
}

pub fn collection_from_file(fname: &str) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut games = Vec::new();

    let table = TableEntry::table_from_file(fname).expect("Failed to open data file!");
    for game in table {
        games.push(from_table_entry(&game));
    }

    games
}
