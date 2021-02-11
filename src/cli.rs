use crate::font::{Font, FontContainer};
use std::io;
use std::path::Path;

pub fn print_font_info(input_path: &str, ttc_indices: &Vec<usize>) -> io::Result<()> {
    let font_container = FontContainer::read(input_path)?;
    let font_num = font_container.fonts.len();
    let indent = "    ";
    let init = || println!("Listing table info for {:?}:\n", input_path);
    match font_num {
        0 => eprintln!("Invalid font files."),
        1 => {
            if !ttc_indices.is_empty() {
                eprintln!("WARNING: Your font number specification will be ignored.");
            }
            init();
            println!("{}\n", font_container.fonts[0].fmt_font_info(indent));
        }
        _ => {
            init();
            let file_name = Path::new(input_path).file_name().unwrap().to_str().unwrap();
            let print_font = |(i, font): (usize, &Font)| {
                println!("{}#{}:\n{}\n", file_name, i, font.fmt_font_info(indent))
            };
            if ttc_indices.is_empty() {
                font_container.fonts.iter().enumerate().for_each(print_font);
            } else {
                let max_index = font_container.fonts.len() - 1;
                let index_error = |i: usize| {
                    eprintln!(
                        "The font number should be between 0 and {}, but you specify {}.",
                        max_index, i
                    )
                };
                ttc_indices
                    .iter()
                    .for_each(|&i| match font_container.fonts.get(i) {
                        Some(font) => print_font((i, font)),
                        _ => index_error(i),
                    })
            }
        }
    }
    Ok(())
}

pub fn print_tables(
    input_path: &str,
    ttc_indices: &Vec<usize>,
    tables: &Vec<&str>,
) -> io::Result<()> {
    let mut font_container = FontContainer::read(input_path)?;
    let font_num = font_container.fonts.len();
    let init = || println!("Dumping {:?}:\n", input_path);
    // TODO: don't parse all the tables
    font_container.parse();
    match font_num {
        0 => eprintln!("Invalid font files."),
        1 => {
            if !ttc_indices.is_empty() {
                eprintln!("WARNING: Your font number specification will be ignored.");
            }
            init();
            println!("{}", font_container.fonts[0].fmt_tables(&tables));
        }
        _ => {
            init();
            let file_name = Path::new(input_path).file_name().unwrap().to_str().unwrap();
            let print_font = |(i, font): (usize, &Font)| {
                println!("{}#{}:\n{}", file_name, i, font.fmt_tables(&tables));
            };
            if ttc_indices.is_empty() {
                font_container.fonts.iter().enumerate().for_each(print_font);
            } else {
                let max_index = font_container.fonts.len() - 1;
                let index_error = |i: usize| {
                    eprintln!(
                        "The font number should be between 0 and {}, but you specify {}.",
                        max_index, i
                    )
                };
                ttc_indices
                    .iter()
                    .for_each(|&i| match font_container.fonts.get(i) {
                        Some(font) => print_font((i, font)),
                        _ => index_error(i),
                    })
            }
        }
    }
    Ok(())
}
