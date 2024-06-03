use std::fs::File;

use chrono::NaiveDate;
use wa_quotes::{Message, QuoteMessage};

fn main() {
    let messages: Vec<Message> =
        serde_json::from_reader(File::open("messages.json").unwrap()).unwrap();

    serde_json::to_writer(
        File::create("quotes.json").unwrap(),
        &messages
            .into_iter()
            .filter(|m| m.time > NaiveDate::from_ymd_opt(2023, 05, 21).unwrap().into())
            .filter(|m| m.content.len() < 100)
            .flat_map(|q| q.try_into().ok())
            .collect::<Vec<QuoteMessage>>(),
    )
    .unwrap();
}
