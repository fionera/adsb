// Script to extract airline codes and descriptions from https://www.eurocontrol.int/rmalive/regulatorListInit.do
// it returns the current column layout on stderr and the data itself on stdout. everything is encoded as TSV

use std::collections::HashMap;
use std::io::Cursor;
use calamine::{DataType, open_workbook_from_rs, Reader, Xls};
use scraper::{Html, Selector};
use lazy_static::lazy_static;
use reqwest::{Client};

lazy_static! {
    static ref OPTION: Selector = Selector::parse("option").unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let countries = fetch_countries().await?;

    let mut params:HashMap<String, String> = HashMap::new();
    params.insert(String::from("action"), String::from("search"));

    let client = Client::builder().cookie_store(true).build()?;

    for country in countries {
        eprintln!("Fetching {}", country);
        params.insert(String::from("operatorState"), country.clone());

        let request = client.post("https://www.eurocontrol.int/rmalive/regulatorList.do")
            .form(&params)
            .build()?;
        client.execute(request).await?.text().await?;

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

        rows.iter().take(1).for_each(|row| eprintln!("COUNTRY\t{}", row.join("\t")));
        rows.iter().skip(1).for_each(|row| println!("{}\t{}", country, row.join("\t")));
    }

    Ok(())
}

async fn fetch_countries<'str>() -> Result<Vec<String>, reqwest::Error> {
    let resp = reqwest::get("https://www.eurocontrol.int/rmalive/regulatorList.do").await?.text().await?;
    let document = Html::parse_document(&resp);
    let countries = document.select(&OPTION)
        .map(|x| x.value().attr("value").unwrap())
        .filter(|x| !x.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();

    Ok(countries)
}

fn parse_row(row: &[DataType]) -> Vec<String> {
    row.iter()
        .map(|column| {
            let value = match column {
                DataType::String(v) => v.to_string(),
                DataType::Bool(v) => v.to_string(),
                DataType::Int(_) |
                DataType::Float(_) |
                DataType::DateTime(_) => column.as_date().unwrap().to_string(),
                DataType::Error(e) => panic!("{}", e),
                DataType::Empty => String::from("")
            };
            let value = value.replace('\0', "");
            let value = value.trim();
            String::from(value)
        })
        .collect::<Vec<_>>()
}