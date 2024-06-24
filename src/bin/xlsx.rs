use std::fs::File;

use rust_xlsxwriter::Format;
use wa_quotes::QuoteMessage;

fn main() {
    let quotes: Vec<QuoteMessage> =
        serde_json::from_reader(File::open("quotes.json").unwrap()).unwrap();

    let mut workbook = rust_xlsxwriter::Workbook::new();
    let worksheet = workbook.add_worksheet();

    let bold = Format::new().set_bold();

    worksheet
        .write_string_with_format(0, 0, "Date/Time", &bold)
        .unwrap();
    worksheet
        .write_string_with_format(0, 1, "Sender", &bold)
        .unwrap();
    let max_quotes = quotes.iter().map(|q| q.quotes.len()).max().unwrap_or(1) as u16;
    for i in 0..max_quotes {
        let i = i + 1;
        worksheet
            .write_string_with_format(0, 2 * i, format!("Quoted {}", i), &bold)
            .unwrap();
        worksheet
            .write_string_with_format(0, 2 * i + 1, format!("Quote {}", i), &bold)
            .unwrap();
    }
    let mut next = 2 * max_quotes;
    for &name in &["Pars", "Johan", "Julia", "Iza", "Julius"] {
        worksheet.write_string_with_format(0, next, name, &bold).unwrap();
        next += 1;
    }

    let dt_format = Format::new().set_num_format("yyyy-mm-dd hh:mm:ss");

    for (row, q) in quotes.into_iter().enumerate() {
        let row = row + 1;
        worksheet
            .write_datetime_with_format(row as u32, 0, q.time, &dt_format)
            .unwrap();
        worksheet.write_string(row as u32, 1, &q.sender).unwrap();
        for (i, quote) in q.quotes.into_iter().enumerate() {
            let i = i as u16 + 1;
            worksheet
                .write_string(row as u32, 2 * i, &quote.quoted)
                .unwrap();
            worksheet
                .write_string(row as u32, 2 * i + 1, &quote.quote)
                .unwrap();
        }
    }

    workbook.save("quotes.xlsx").unwrap();
}
