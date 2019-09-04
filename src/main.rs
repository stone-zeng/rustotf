// use std::env;
// use std::process;

// use rustotf;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let font_file_name = rustotf::parse_args(&args).unwrap_or_else(|e| {
//         eprintln!("Problem parsing arguments: {}", e);
//         process::exit(1);
//     });
//     if let Err(e) = rustotf::run(font_file_name) {
//         eprintln!("Application error: {}", e);
//         process::exit(1);
//     }
// }

use rustotf;
use std::process;
// use std::mem;
// use std::io::prelude::*;

fn main() {
    for font_file_name in &[
        // "./assets/iosevka-regular.ttc",
        // "./assets/NotoColorEmoji.ttf",
        // "./assets/SourceHan.ttc",
        // "./assets/SourceSansPro-Regular.otf.woff2",
        "./assets/SourceSansPro-Regular.otf",
        // "./assets/SourceSansPro-Regular.ttf.woff",
        // "./assets/SourceSansPro-Regular.ttf",
        // "./assets/SourceSansVariable-Roman.otf.woff",
        // "./assets/SourceSansVariable-Roman.otf.woff2",
        // "./assets/SourceSansVariable-Roman.otf",
        // "/Applications/Adobe Illustrator CC 2019/Adobe Illustrator.app/Contents/Required/Fonts/KozGoPr6N-Regular.otf",
        // "/Applications/Microsoft Excel.app/Contents/Resources/DFonts/Cambria.ttc",
        // "/Applications/Microsoft Excel.app/Contents/Resources/DFonts/times.ttf",
        // "/Library/Fonts/Apple Chancery.ttf",
        // "/Library/Fonts/Didot.ttc",
        // "/Library/Fonts/FiraGO-Regular.otf",
        // "/System/Library/Fonts/ヒラギノ角ゴシック W4.ttc",
    ] {
        if let Err(e) = rustotf::run(font_file_name) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }
}
