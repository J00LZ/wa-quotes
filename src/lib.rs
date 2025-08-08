use std::str::FromStr;

use rust_xlsxwriter::XlsxSerialize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub time: chrono::NaiveDateTime,
    pub sender: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quoted: Option<String>,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, XlsxSerialize)]
pub struct QuoteMessage {
    #[xlsx(rename = "Time")]
    pub time: chrono::NaiveDateTime,
    #[xlsx(rename = "Sender")]
    pub sender: String,
    #[xlsx(rename = "Quotes")]
    pub quotes: Vec<Quote>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Quote {
    pub quoted: String,
    pub quote: String,
}

impl FromStr for Message {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let time = chrono::NaiveDateTime::parse_from_str(&s[1..21], "%d/%m/%Y, %H:%M:%S").unwrap();
        let new_s = &s[22..];
        let sender = new_s[..new_s.find(':').unwrap()].trim().to_string();
        let new_s = &new_s[new_s.find(':').unwrap() + 1..];
        let mut content = new_s.trim().to_string();
        if let Some(f) = content.find('\u{200E}') {
            content.replace_range(f.., "");
            content = content.trim().to_string();
        }
        Ok(Message {
            time,
            sender,
            quoted: None,
            content,
        })
    }
}

impl TryFrom<Message> for QuoteMessage {
    type Error = ();

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        let mut quotes = vec![];
        for line in value.content.lines() {
            if let Some((quoted, quote)) = line.split_once(": \"") {
                let last_quote_index = quote.rfind('"');
                if let Some(idx) = last_quote_index {
                    quotes.push(Quote {
                        quoted: quoted.to_string().trim().to_string(),
                        quote: quote[..idx].trim().to_string(),
                    });
                }
            }
        }
        if quotes.is_empty() {
            return Err(());
        }
        Ok(QuoteMessage {
            time: value.time,
            sender: value.sender,
            quotes,
        })
    }
}
