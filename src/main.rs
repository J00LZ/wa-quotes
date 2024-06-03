use std::fs::File;

use docx_rs::{Docx, LineSpacing, Paragraph, Run, RunFonts, Styles};
use wa_quotes::QuoteMessage;

fn main() {
    let quotes: Vec<QuoteMessage> =
        serde_json::from_reader(File::open("quotes.json").unwrap()).unwrap();
    let mut docx = Docx::new();
    docx = docx.styles(Styles::new().default_fonts(RunFonts::new().ascii("Arial")));
    for q in &quotes {
        let mut p = Paragraph::new();
        for q in &q.quotes {
            let mut r = Run::new().bold();
            r = r.add_text(&q.quoted);
            p = p.add_run(r);
            p = p.add_run(Run::new().add_text(": "));
            p = p.add_run(
                Run::new()
                    .add_text(&q.quote)
                    .add_break(docx_rs::BreakType::TextWrapping),
            );
        }
        p = p.line_spacing(LineSpacing::new().after_lines(100)).add_run(
            Run::new()
                .bold()
                .add_text("By ")
                .add_text(&q.sender)
                .add_text(" on ")
                .add_text(&format!("{}", q.time.format("%Y-%m-%d %H:%M:%S"))),
        );
        docx = docx.add_paragraph(p);
    }
    let docx = docx.build();
    let file = File::create("test.docx").unwrap();
    docx.pack(file).unwrap();
}
