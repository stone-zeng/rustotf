pub mod cli;
mod font;
mod types;
mod util;
mod tables {
    pub mod required {
        pub mod cmap;
        pub mod head;
        pub mod hhea;
        pub mod hmtx;
        pub mod maxp;
        pub mod name;
        pub mod os_2;
        pub mod post;
    }
    pub mod ttf {
        pub mod cvt_;
        pub mod fpgm;
        pub mod gasp;
        pub mod glyf;
        pub mod loca;
        pub mod prep;
    }
    pub mod cff {
        pub mod cff_;
        // pub mod cff2;
        mod cff_char_string;
        mod cff_data;
        pub mod vorg;
    }
    pub mod bitmap {
        pub mod ebdt;
        pub mod eblc;
        pub mod ebsc;
    }
    pub mod layout {
        pub mod base;
        pub mod gsub;
        pub mod jstf;
        pub mod math;
    }
    pub mod otvar {
        pub mod avar;
        pub mod fvar;
        pub mod hvar;
        pub mod mvar;
    }
    pub mod color {
        pub mod cbdt;
        pub mod cblc;
        pub mod colr;
        pub mod cpal;
        pub mod sbix;
        pub mod svg_;
    }
    pub mod other {
        pub mod dsig;
        pub mod ltsh;
    }
}

pub use font::{Font, FontContainer};
pub use types::Tag;

#[rustfmt::skip]
pub use tables::{
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
        vorg::Table_VORG,
    },
    bitmap::{
        ebdt::Table_EBDT,
        eblc::Table_EBLC,
        ebsc::Table_EBSC,
    },
    layout::{
        base::Table_BASE,
        gsub::Table_GSUB,
        jstf::Table_JSTF,
        math::Table_MATH,
    },
    otvar::{
        avar::Table_avar,
        fvar::Table_fvar,
        hvar::Table_HVAR,
        mvar::Table_MVAR,
    },
    color::{
        colr::Table_COLR,
        cpal::Table_CPAL,
        cbdt::Table_CBDT,
        cblc::Table_CBLC,
        sbix::Table_sbix,
        svg_::Table_SVG_,
    },
    other::{
        dsig::Table_DSIG,
        ltsh::Table_LTSH,
    },
};
