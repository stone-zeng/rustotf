use rustotf;
use std::process;

fn main() {
    for font_file_name in &[
        // Basic
        "./test/fonts/macos/Arial.ttf",
        "./test/fonts/msword/times.ttf",
        "./test/fonts/texlive/lmroman10-regular.otf",
        "./test/fonts/texlive/NotoSans-Regular.ttf",
        "./test/fonts/texlive/SourceSansPro-RegularIt.otf",
        // CJK
        "./test/fonts/contrib/BabelStoneHan.ttf",
        "./test/fonts/contrib/BabelStoneHanPUA.ttf",
        "./test/fonts/contrib/HYZhongHeiS.ttf",
        "./test/fonts/msword/msyh.ttf",
        "./test/fonts/texlive/FandolSong-Regular.otf",
        // TTC
        "./test/fonts/contrib/iosevka-medium.ttc",
        "./test/fonts/contrib/sarasa-regular.ttc",
        "./test/fonts/contrib/SourceHanNotoCJK.ttc",
        "./test/fonts/macos/HelveticaNeue.ttc",
        "./test/fonts/macos/PingFang.ttc",
        "./test/fonts/macos/ヒラギノ角ゴシック W4.ttc",
        "./test/fonts/msword/Cambria.ttc",
        // Emoji
        "./test/fonts/contrib/NotoColorEmoji.ttf",
        "./test/fonts/contrib/NotoEmoji-Regular.ttf",
        "./test/fonts/macos/Apple Color Emoji.ttc",
        // Variable fonts
        "./test/fonts/contrib/AdobeVFPrototype.otf",
        "./test/fonts/contrib/AdobeVFPrototype.ttf",
        "./test/fonts/macos/SFNS.ttf",
        // WOFF
        // WOFF2
    ] {
        if let Err(e) = rustotf::read_font(font_file_name) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }
}
