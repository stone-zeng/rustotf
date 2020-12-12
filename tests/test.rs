use rustotf::FontContainer;
use std::fs;

const FONTS_PATH: &str = "./tests/fonts/";

const TTF_FONTS: &[&str] = &[
    "cozette_bitmap.ttf",
    "NotoSans-SemiCondensed.ttf",
    "SourceSerifPro-LightIt.ttf",
    "WorkSans-Regular.ttf",
];

const OTF_FONTS: &[&str] = &[
    "emojione-svg.otf",
    "SourceHanSansSC-Regular.otf",
    "SourceSans3-Black.otf",
    "XITSMath-Regular.otf",
];

const TTC_FONTS: &[&str] = &[
    "SourceHanSans-Heavy.ttc",
    "SourceHanSerif-Regular.ttc",
    "SourceHanNotoCJK.ttc",
];

const WOFF_FONTS: &[&str] = &[
    "SourceSans3-ExtraLight.ttf.woff",
    "SourceSans3VF-Roman.ttf.woff",
    "SourceSerifPro-Bold.otf.woff",
    "SourceSerifVariable-Italic.otf.woff",
];

fn check_font(font_file_path: &str, flag: &str) {
    println!("Checking font: {}", font_file_path);

    let mut font_container = FontContainer::new(fs::read(font_file_path).unwrap());
    font_container.init();
    assert_ne!(font_container.fonts.len(), 0);

    font_container.parse();

    for font in &font_container.fonts {
        assert!(font.head.is_some());
        assert!(font.hhea.is_some());
        assert!(font.maxp.is_some());
        assert!(font.hmtx.is_some());
        assert!(font.cmap.is_some());
        assert!(font.name.is_some());
        assert!(font.OS_2.is_some());
        assert!(font.post.is_some());
    }

    match flag {
        "ttf" => {
            for font in &font_container.fonts {
                assert!(font.loca.is_some());
                assert!(font.glyf.is_some());
            }
        }
        "otf" => {
            for font in &font_container.fonts {
                assert!(font.CFF_.is_some());
            }
        }
        _ => ()
    }
}

#[test]
fn check_ttf() {
    for i in TTF_FONTS {
        let font_file_name = [FONTS_PATH, i].join("");
        check_font(&font_file_name, "ttf");
    }
}

#[test]
fn check_otf() {
    for i in OTF_FONTS {
        let font_file_name = [FONTS_PATH, i].join("");
        check_font(&font_file_name, "otf");
    }
}

#[test]
fn check_ttc() {
    for i in TTC_FONTS {
        let font_file_name = [FONTS_PATH, i].join("");
        check_font(&font_file_name, "");
    }
}

#[test]
fn check_woff() {
    for i in WOFF_FONTS {
        let font_file_name = [FONTS_PATH, i].join("");
        check_font(&font_file_name, "");
    }
}
