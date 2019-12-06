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
    pub mod avar;
    pub mod fvar;
}

pub use font::{
    read_font,
    Font,
    FontContainer,
};

pub use table::{
    cmap::Table_cmap,
    head::Table_head,
    hhea::Table_hhea,
    hmtx::Table_hmtx,
    maxp::Table_maxp,
    name::Table_name,
    os_2::Table_OS_2,
    post::Table_post,
    avar::Table_avar,
    fvar::Table_fvar,
};

// pub fn parse_args(args: &[String]) -> Result<&str, &str> {
//     if args.len() > 1 {
//         Ok(&args[1])
//     } else {
//         Err("not enough arguments")
//     }
// }
