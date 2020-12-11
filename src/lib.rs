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
    pub mod cff {
        pub mod cff_;
        // pub mod cff2;
        // pub mod vorg;
    }
    pub mod bitmap {
        pub mod ebdt;
        pub mod eblc;
        pub mod ebsc;
    }
    pub mod otvar {
        pub mod avar;
        pub mod fvar;
        pub mod hvar;
        pub mod mvar;
    }
    pub mod color {
        // pub mod COLR;
        // pub mod CPAL;
        // pub mod CBDT;
        // pub mod CBLC;
        pub mod sbix;
        pub mod svg_;
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
    cff::{
        cff_::Table_CFF_,
        // cff2::Table_CFF2,
        // vorg::Table_VORG,
    },
    bitmap::{
        ebdt::Table_EBDT,
        eblc::Table_EBLC,
        ebsc::Table_EBSC,
    },
    otvar::{
        avar::Table_avar,
        fvar::Table_fvar,
        hvar::Table_HVAR,
        mvar::Table_MVAR,
    },
    color::{
        // COLR::Table_COLR,
        // CPAL::Table_CPAL,
        // CBDT::Table_CBDT,
        // CBLC::Table_CBLC,
        sbix::Table_sbix,
        svg_::Table_SVG_,
    },
};

// pub fn parse_args(args: &[String]) -> Result<&str, &str> {
//     if args.len() > 1 {
//         Ok(&args[1])
//     } else {
//         Err("not enough arguments")
//     }
// }
