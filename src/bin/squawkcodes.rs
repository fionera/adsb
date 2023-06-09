// Script to extract squawk codes and descriptions from https://www.flightradars.eu/squawkcodes.html
// it returns the current column layout on stderr and the data itself on stdout. everything is encoded as TSV

use scraper::{Html, Selector};
use lazy_static::lazy_static;

lazy_static! {
    // they have a very weird website layout. we just take the lowest tr that exists
    static ref TABLE_ROW: Selector = Selector::parse("table > tbody > tr > td > table > tbody > tr").unwrap();
    static ref TD: Selector = Selector::parse("td").unwrap();
}

fn parse_number(s: &str) -> Vec<i32> {
    let s = s.trim_end_matches('.');
    if s.contains('-') {
        // split by `-` and parse numbers to int
        let (start, end) = s.split_once('-').unwrap();
        let start = start.parse::<i32>().expect("parsing of number failed");
        let end = end.parse::<i32>().expect("parsing of number failed");

        // create range
        return (start..end).collect::<Vec<_>>();
    }

    vec![s.parse::<i32>().expect("parsing of number failed")]
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://www.flightradars.eu/squawkcodes.html")
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&resp);
    let entries = document.select(&TABLE_ROW)
        // filter for data rows
        .filter(|row| row.select(&TD).count() == 2)

        // flatmap returning entries. we return a vec because we directly unwrap
        // the entries that contain a range
        .flat_map(|row| {
            let mut columns = row.select(&TD).collect::<Vec<_>>();

            let description = columns.pop().unwrap().text().collect::<Vec<_>>().pop().unwrap();
            let numbers = parse_number(&columns.pop().unwrap().inner_html());

            numbers.iter().map(|number| (number.abs(), description.to_owned())).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    eprintln!("Code\tDescription");
    entries.iter().for_each(|(number, description)| {
        println!("{}\t{}", number, description);
    });

    Ok(())
}