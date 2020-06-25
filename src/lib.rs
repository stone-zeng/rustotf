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
    // pub mod cvt_;
    // pub mod fpgm;
    pub mod glyf;
    pub mod loca;
    // pub mod prep;
    // pub mod gasp;
    pub mod avar;
    // pub mod cvar;
    pub mod fvar;
    // pub mod gvar;
    pub mod hvar;
    pub mod mvar;
    // pub mod stat;
    // pub mod vvar;
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
    // cvt_::Table_cvt_,
    // fpgm::Table_fpgm,
    glyf::Table_glyf,
    loca::Table_loca,
    // prep::Table_prep,
    // gasp::Table_gasp,
    avar::Table_avar,
    // cvar::Table_cvar,
    fvar::Table_fvar,
    // gvar::Table_gvar,
    hvar::Table_HVAR,
    mvar::Table_MVAR,
    // stat::Table_STAT,
    // vvar::Table_VVAR,
};

// pub fn parse_args(args: &[String]) -> Result<&str, &str> {
//     if args.len() > 1 {
//         Ok(&args[1])
//     } else {
//         Err("not enough arguments")
//     }
// }
