use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::BufWriter,
};

use wa_quotes::QuoteMessage;

#[derive(Debug, serde::Serialize)]
struct Stats {
    quotes_by: BTreeMap<usize, Vec<String>>,
    total_quotes: usize,
    quoted: BTreeMap<usize, Vec<String>>,
}

fn main() {
    let quotes: Vec<QuoteMessage> =
        serde_json::from_reader(File::open("quotes.json").unwrap()).unwrap();
    let mut stats = Stats {
        quotes_by: BTreeMap::new(),
        total_quotes: 0,
        quoted: BTreeMap::new(),
    };

    let mut quotes_by = HashMap::<_, u32>::new();
    let mut quoted = HashMap::<_, u32>::new();

    for quote in quotes {
        stats.total_quotes += quote.quotes.len();
        *quotes_by.entry(quote.sender.clone()).or_default() += 1;
        for q in quote.quotes {
            *quoted.entry(q.quoted).or_default() += 1;
        }
    }

    for (k, v) in quotes_by {
        stats.quotes_by.entry(v as usize).or_default().push(k);
    }

    for (k, v) in quoted {
        stats.quoted.entry(v as usize).or_default().push(k);
    }

    serde_json::to_writer_pretty(
        BufWriter::new(File::create("./stats.json").unwrap()),
        &stats,
    )
    .unwrap();
}
