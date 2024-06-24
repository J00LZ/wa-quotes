// use std::fs::File;

// use fontdue::{
//     layout::{Layout, LayoutSettings, TextStyle},
//     FontSettings,
// };
// // use image::{DynamicImage, GenericImage, GenericImageView as _, Pixel};

// use wa_quotes::QuoteMessage;

// fn main() {
//     let quotes: Vec<QuoteMessage> =
//         serde_json::from_reader(File::open("quotes.json").unwrap()).unwrap();
//     // let mut quotees = HashMap::new();
//     // for q in &quotes {
//     //     *quotees.entry(&q.sender).or_insert(0)+=1;
//     // }

//     // let mut quoted = HashMap::new();
//     // for q in &quotes {
//     //     for quote in &q.quotes {
//     //         *quoted.entry(&quote.quoted).or_insert(0)+=1;
//     //     }
//     // }

//     // let most_quoted = quoted.iter().max_by_key(|(_, v)| *v).unwrap().0;
//     // let most_quotes = quotees.iter().max_by_key(|(_, v)| *v).unwrap().0;
//     // println!("Most quoted: {}", most_quoted);
//     // println!("Most quotes: {}", most_quotes);
//     let background = image::open("./bg.png").unwrap();
//     let img = image::DynamicImage::ImageRgba8(background.to_rgba8());
//     for quote in quotes {
//         let mut img = img.clone();

//         let secs = quote.time.and_utc().timestamp();

//         println!("Now doing {}", quote.time);

//         let filename = format!(
//             "out/{} - {} - {}.png",
//             quote.sender,
//             quote
//                 .quotes
//                 .iter()
//                 .map(|q| q.quoted.clone())
//                 .collect::<Vec<_>>()
//                 .join(","),
//             secs
//         );
//         draw_text(&quote, &mut img);

//         img.save(filename).unwrap();
//     }
// }

// fn draw_text(quote: &QuoteMessage, img: &mut DynamicImage) {
//     let font = include_bytes!("../../fonts/Roboto-Regular.ttf") as &[u8];
//     let font = fontdue::Font::from_bytes(font, FontSettings::default()).unwrap();
//     let font2 = include_bytes!("../../fonts/Roboto-Bold.ttf") as &[u8];
//     let font2 = fontdue::Font::from_bytes(font2, FontSettings::default()).unwrap();
//     let fonts = &[font, font2];
//     let mut layout = Layout::<()>::new(fontdue::layout::CoordinateSystem::PositiveYDown);
//     layout.reset(&LayoutSettings {
//         x: 96.0,
//         y: 118.0,
//         max_width: Some(900.0),
//         max_height: Some(475.0),
//         horizontal_align: fontdue::layout::HorizontalAlign::Center,
//         vertical_align: fontdue::layout::VerticalAlign::Middle,
//         ..LayoutSettings::default()
//     });
//     let scale = 56.0;
//     let q = &quote.quotes;
//     let total_lines = q.len();
//     for (idx, quote) in q.iter().enumerate() {
//         layout.append(fonts, &TextStyle::new(&quote.quoted, scale, 1));
//         layout.append(fonts, &TextStyle::new(": ", scale, 0));
//         layout.append(fonts, &TextStyle::new(&quote.quote, scale, 0));
//         if idx < total_lines - 1 {
//             layout.append(fonts, &TextStyle::new("\n", scale, 0));
//         }
//     }
//     for g in layout.glyphs() {
//         if g.char_data.is_control() || g.char_data.is_whitespace() {
//             continue;
//         }
//         let (metrics, data) = fonts[g.font_index].rasterize_indexed(g.key.glyph_index, g.key.px);
//         let start_x = g.x;
//         let start_y = g.y;
//         for y in 0..metrics.height {
//             let pixely = start_y as i32 + y as i32;
//             for x in 0..metrics.width {
//                 let pixelx = start_x as i32 + x as i32;
//                 let alpha = data[y * metrics.width + x];
//                 let mut pixel = img.get_pixel(pixelx as u32, pixely as u32);
//                 let new = image::Rgba([0, 0, 0, alpha]);
//                 pixel.blend(&new);
//                 img.put_pixel(pixelx as u32, pixely as u32, pixel);
//             }
//         }
//     }
//     let scale = 56.0;
//     layout.reset(&LayoutSettings {
//         x: 45.0,
//         y: 951.0,
//         max_width: Some(900.0),
//         max_height: Some(1080.0 - 951.0),
//         horizontal_align: fontdue::layout::HorizontalAlign::Left,
//         vertical_align: fontdue::layout::VerticalAlign::Middle,
//         ..LayoutSettings::default()
//     });
//     layout.append(fonts, &TextStyle::new(&quote.sender, scale, 1));

//     for g in layout.glyphs() {
//         if g.char_data.is_control() || g.char_data.is_whitespace() {
//             continue;
//         }
//         let (metrics, data) = fonts[g.font_index].rasterize_indexed(g.key.glyph_index, g.key.px);
//         let start_x = g.x;
//         let start_y = g.y;
//         for y in 0..metrics.height {
//             let pixely = start_y as i32 + y as i32;
//             for x in 0..metrics.width {
//                 let pixelx = start_x as i32 + x as i32;
//                 let alpha = data[y * metrics.width + x];
//                 let mut pixel = img.get_pixel(pixelx as u32, pixely as u32);
//                 let new = image::Rgba([0, 0, 0, alpha]);
//                 pixel.blend(&new);
//                 img.put_pixel(pixelx as u32, pixely as u32, pixel);
//             }
//         }
//     }
// }

fn main() {
    
}
