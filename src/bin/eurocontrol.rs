// Script to extract airline codes and descriptions from https://www.eurocontrol.int/rmalive/regulatorExport.do
// it returns the current column layout on stderr and the data itself on stdout. everything is encoded as TSV

use std::collections::HashMap;
use std::io::Cursor;
use calamine::{DataType, open_workbook_from_rs, Reader, Xls};
use scraper::{Selector};
use lazy_static::lazy_static;
use reqwest::{Client};

lazy_static! {
    static ref OPTION: Selector = Selector::parse("option").unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;


    let mut params = HashMap::new();
    params.insert("action", "search");

    let request = client.get("https://www.eurocontrol.int/rmalive/regulatorExport.do?type=xls")
        .build()?;
    let response = client.execute(request).await?.bytes().await?;


    let mut workbook: Xls<_> = open_workbook_from_rs(Cursor::new(response))?;
    let sheets = workbook.worksheets();
    let (_, range) = sheets.first().unwrap();

    let rows = range.rows()
        .skip(2)// skip description rows
        .map(parse_row)
        .collect::<Vec<_>>();

    rows.iter().take(1).for_each(|row|eprintln!("{}", row.join("\t")));
    rows.iter().skip(1).for_each(|row| println!("{}", row.join("\t")));

    Ok(())
}

fn parse_row(row: &[DataType]) -> Vec<String> {
    row.iter()
        .map(|column| {
            let value = match column {
                DataType::Int(v) => v.to_string(),
                DataType::Float(v) => v.to_string(),
                DataType::String(v) => v.to_string(),
                DataType::Bool(v) => v.to_string(),
                DataType::DateTime(v) => v.to_string(),
                DataType::Error(e) => panic!("{}", e),
                DataType::Empty => String::from("")
            };
            let value = value.replace('\0', "");
            let value = value.trim();
            String::from(value)
        })
        .collect::<Vec<_>>()
}