use rustotf::FontContainer;
use std::io::Result;

const FONTS_PATH: &str = "./tests/fonts/";

const TTF_FONTS: &[&str] = &[
    "AdobeVFPrototype.ttf",
    "cozette_bitmap.ttf",
    "NotoColorEmoji.ttf",
    "NotoSans-SemiCondensed.ttf",
    "SourceSerif4-LightIt.ttf",
    "TwemojiMozilla.ttf",
    "WorkSans-Regular.ttf",
];

const OTF_FONTS: &[&str] = &[
    "AdobeVFPrototype.otf",
    "CJKRadicals-Regular.otf",
    "emojione-svg.otf",
    "FDArrayTest257.otf",
    "FDArrayTest65535.otf",
    "SourceHanSansSC-Regular.otf",
    "SourceSans3-Black.otf",
    "XITSMath-Regular.otf",
];

const TTC_FONTS: &[&str] = &[
    "iosevka.ttc",
    "SourceHanNotoCJK.ttc",
    "SourceHanSans-Heavy.ttc",
    "SourceHanSerif-Regular.ttc",
    "SourceHanVFProto.ttc",
];

const WOFF_FONTS: &[&str] = &[
    "SourceSans3-ExtraLight.ttf.woff",
    "SourceSans3VF-Roman.ttf.woff",
    "SourceSerif4Display-Bold.otf.woff",
    "SourceSerif4Variable-Italic.otf.woff",
];

#[allow(dead_code)]
const WOFF2_FONTS: &[&str] = &[
    "SourceCodePro-Medium.otf.woff2",
    "SourceCodeVariable-Italic.ttf.woff2",
];

fn check_font(font_file_path: &str, flag: &str) -> Result<()> {
    println!("Checking font: {}", font_file_path);

    let mut font_container = FontContainer::read(font_file_path)?;
    assert_ne!(font_container.len(), 0);

    font_container.parse();

    macro_rules! _assert {
        ($font:ident, $t:ident, $s:expr) => {
            if $font.contains($s) {
                println!("{}", $s);
                assert!($font.$t.is_some())
            }
        };
        ($font:ident, $t:ident) => {
            _assert!($font, $t, stringify!($t))
        };
    }

    font_container.into_iter().for_each(|font| {
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
            font_container.into_iter().for_each(|font| {
                _assert!(font, loca);
                _assert!(font, glyf);
            });
        }
        "otf" => {
            font_container.into_iter().for_each(|font| {
                _assert!(font, CFF_, "CFF ");
            });
        }
        _ => (),
    }

    Ok(())
}

#[test]
fn check_ttf() -> Result<()> {
    for i in TTF_FONTS {
        let font_file_name = [FONTS_PATH, i].join("");
        check_font(&font_file_name, "ttf")?;
    }
    Ok(())
}

#[test]
fn check_otf() -> Result<()> {
    for i in OTF_FONTS {
        let font_file_name = [FONTS_PATH, i].join("");
        check_font(&font_file_name, "otf")?;
    }
    Ok(())
}

#[test]
fn check_ttc() -> Result<()> {
    for i in TTC_FONTS {
        let font_file_name = [FONTS_PATH, i].join("");
        check_font(&font_file_name, "")?;
    }
    Ok(())
}

#[test]
fn check_woff() -> Result<()> {
    for i in WOFF_FONTS {
        let font_file_name = [FONTS_PATH, i].join("");
        check_font(&font_file_name, "")?;
    }
    Ok(())
}
