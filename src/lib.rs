mod font;
mod util;
mod table {
    pub mod required {
        pub mod head;
        pub mod hhea;
        pub mod maxp;
        pub mod hmtx;
        pub mod cmap;
        pub mod name;
        pub mod os_2;
        pub mod post;
    }
    pub mod ttf {
        pub mod loca;
        pub mod glyf;
        pub mod cvt_;
        pub mod fpgm;
        pub mod prep;
        pub mod gasp;
    }
    pub mod otvar {
        pub mod avar;
        pub mod fvar;
        pub mod hvar;
        pub mod mvar;
    }
}

pub use font::{
    read_font,
    Font,
    FontContainer,
};

pub use table::{
    required::{
        head::Table_head,
        hhea::Table_hhea,
        maxp::Table_maxp,
        hmtx::Table_hmtx,
        cmap::Table_cmap,
        name::Table_name,
        os_2::Table_OS_2,
        post::Table_post,
    },
    ttf::{
        loca::Table_loca,
        glyf::Table_glyf,
        cvt_::Table_cvt_,
        fpgm::Table_fpgm,
        prep::Table_prep,
        gasp::Table_gasp,
    },
    otvar::{
        avar::Table_avar,
        fvar::Table_fvar,
        hvar::Table_HVAR,
        mvar::Table_MVAR,
    }
};

// pub fn parse_args(args: &[String]) -> Result<&str, &str> {
//     if args.len() > 1 {
//         Ok(&args[1])
//     } else {
//         Err("not enough arguments")
//     }
// }
