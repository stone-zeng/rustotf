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

pub use font::read_font;

pub use font::Font;
pub use font::FontContainer;

// pub fn parse_args(args: &[String]) -> Result<&str, &str> {
//     if args.len() > 1 {
//         Ok(&args[1])
//     } else {
//         Err("not enough arguments")
//     }
// }
