use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

use clap::Parser;
use wa_quotes::Message;

#[derive(Debug, clap::Parser)]
struct ParserOpts {
    /// The file to parse. If it ends with `.txt`, it will be parsed as a WhatsApp chat export.
    /// If it ends with `.zip` it will be unzipped and the `_chat.txt` file inside will be parsed.
    file: PathBuf,
}

fn main() {
    let opts = ParserOpts::parse();
    let f = opts.file;
    let f = if f.extension().map(|e| e == "zip").unwrap_or(false) {
        let mut archive = zip::ZipArchive::new(File::open(f).unwrap()).unwrap();
        let dir = tempfile::tempdir().unwrap();
        archive.extract(&dir).unwrap();
        File::open(dir.path().join("_chat.txt")).unwrap()
    } else {
        File::open(f).unwrap()
    };
    let b = BufReader::new(f);
    let mut lines = vec![];
    let mut current = String::new();
    for line in b.lines() {
        let l = line.unwrap();
        // let l = deunicode::deunicode(&l);
        let l = l
            .replace(|c| c == '“' || c == '”', "\"")
            .replace(|c| c == '’', "'");
        if l.starts_with('[') {
            lines.push(
                current
                    .replace(
                        |c| {
                            c == '\u{202A}'
                                || c == '\u{202C}'
                                || c == '\u{200F}'
                                || c == '\u{202F}'
                                || c == '\u{00A0}'
                        },
                        "",
                    )
                    .trim()
                    .to_string(),
            );
            current = l;
        } else {
            current.push('\n');
            current.push_str(&l);
        }
    }
    let messages = lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|l| Message::from_str(&l).unwrap())
        .collect::<Vec<_>>();
    serde_json::to_writer(File::create("messages.json").unwrap(), &messages).unwrap();
}
