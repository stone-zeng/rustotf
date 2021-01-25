use rustotf::FontContainer;
use std::fs;

const FONTS_PATH: &str = "./tests/fonts/";

const TTF_FONTS: &[&str] = &[
    "cozette_bitmap.ttf",
    "NotoColorEmoji.ttf",
    "NotoSans-SemiCondensed.ttf",
    "SourceSerifPro-LightIt.ttf",
    "TwemojiMozilla.ttf",
    "WorkSans-Regular.ttf",
];

const OTF_FONTS: &[&str] = &[
    "CJKRadicals-Regular.otf",
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

    macro_rules! _assert {
        ($font:ident, $t:ident, $s:expr) => {
            if $font.has_table($s) {
                println!("{}", $s);
                assert!($font.$t.is_some())
            }
        };
        ($font:ident, $t:ident) => {
            _assert!($font, $t, stringify!($t))
        };
    }

    font_container.fonts.iter().for_each(|font| {
        _assert!(font, head);
        _assert!(font, hhea);
        _assert!(font, maxp);
        _assert!(font, hmtx);
        _assert!(font, cmap);
        _assert!(font, name);
        _assert!(font, OS_2, "OS/2");
        _assert!(font, post);
    });

    match flag {
        "ttf" => {
            font_container.fonts.iter().for_each(|font| {
                _assert!(font, loca);
                _assert!(font, glyf);
            });
        }
        "otf" => {
            font_container.fonts.iter().for_each(|font| {
                _assert!(font, CFF_, "CFF ");
            });
        }
        _ => (),
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
