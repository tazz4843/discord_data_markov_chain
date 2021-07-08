use crate::model::Record;
use csv::ReaderBuilder;

pub fn parse_csv_file(data: &[u8]) -> Vec<Record> {
    ReaderBuilder::new()
        .from_reader(data)
        .deserialize()
        .filter_map(|i| i.ok())
        .collect()
}
