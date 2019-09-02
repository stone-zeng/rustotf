mod font;
mod util;
mod table {
    pub mod cmap;
    pub mod head;
    pub mod hhea;
    pub mod hmtx;
    pub mod maxp;
    pub mod name;
    pub mod os_2;
    pub mod post;
}

use std::error;
use std::fs;

use font::{read_otf, read_ttc, read_woff, read_woff2};
use util::Buffer;

pub fn parse_args(args: &[String]) -> Result<&str, &str> {
    if args.len() > 1 {
        Ok(&args[1])
    } else {
        Err("not enough arguments")
    }
}

pub fn run(font_file_name: &str) -> Result<(), Box<dyn error::Error>> {
    println!("{:?}", font_file_name);

    let mut buffer = Buffer::new(fs::read(font_file_name)?);

    let signature = buffer.read::<u32>();
    // println!("{:08X}", signature);

    match signature {
        // 'OTTO' | .. | 'true' | 'typ1'
        0x4F54_544F | 0x0001_0000 | 0x7472_7565 | 0x7479_7031 => read_otf(&mut buffer, signature),
        // 'ttcf'
        0x7474_6366 => read_ttc(&mut buffer),
        // 'wOFF'
        0x774F_4646 => read_woff(&mut buffer),
        // 'wOF2'
        0x774F_4632 => read_woff2(&mut buffer),
        _ => (),
    }
    Ok(())
}
