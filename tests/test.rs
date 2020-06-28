use rustotf::FontContainer;
use std::fs;

const FONTS_PATH: &str = "./tests/fonts/";

const TTF_FONTS: [&str; 3] = [
    "NotoSans-SemiCondensed.ttf",
    "SourceSerifPro-LightIt.ttf",
    "WorkSans-Regular.ttf",
];

fn check_font(font_file_path: &str) {
    println!("Checking font: {}", font_file_path);

    let mut font_container = FontContainer::new(fs::read(font_file_path).unwrap());
    font_container.init();
    assert_ne!(font_container.fonts.len(), 0);

    for table in &["head", "hhea", "maxp", "hmtx", "cmap", "name", "OS/2", "post"] {
        font_container.parse_table(table);
    }

    for font in font_container.fonts {
        assert!(font.head.is_some());
        assert!(font.hhea.is_some());
        assert!(font.maxp.is_some());
        assert!(font.hmtx.is_some());
        assert!(font.cmap.is_some());
        assert!(font.name.is_some());
        assert!(font.OS_2.is_some());
        assert!(font.post.is_some());
    }
}

#[test]
fn check_ttf() {
    for i in &TTF_FONTS {
        let font_file_name = [FONTS_PATH, i].join("");
        check_font(&font_file_name);
    }
}
