use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use crate::opts::OutputFormat;

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

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    /*
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
    */
    let mut records = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        // headers.iter() -> 使用 headers 的迭代器
        // record.iter() -> 使用 record 的迭代器
        // zip() -> 将两个迭代器合并为一个元组的迭代器 [(header, record), ..]
        // collect::<Value>() -> 将元组的迭代器转换为 JSON Value
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        records.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&records)?,
        OutputFormat::Yaml => serde_yaml::to_string(&records)?,
        OutputFormat::Toml => toml::to_string(&records)?,
    };

    fs::write(output, content)?;
    Ok(())
}
