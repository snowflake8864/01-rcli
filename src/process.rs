use cfg_if::cfg_if;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    cfg_if! {
        if #[cfg(feature = "collect")] {
            /*
                let records = reader
                .deserialize()
                .map(|record| record.unwrap())
                .collect::<Vec<Player>>();
           */
            let records: Vec<Player> = reader.deserialize().collect::<Result<Vec<_>, _>>()?;
            let json = serde_json::to_string_pretty(&records)?;
        } else {
            let mut records = Vec::with_capacity(128);
            for result in reader.deserialize() {
                let record: Player = result?;
                records.push(record);
            }
        let json = serde_json::to_string_pretty(&records)?;
        }
    }

    fs::write(output, json)?;
    Ok(())
}
