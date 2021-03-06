#!/bin/bash

pwd=${pwd}
gh_url="https://raw.githubusercontent.com"
download="curl --location --output"

cd ./tests/fonts

for url in \
    $gh_url/adobe-fonts/cjk-radicals/master/CJKRadicals-Regular.otf \
    $gh_url/adobe-fonts/fdarray-test/master/FDArrayTest257.otf \
    $gh_url/adobe-fonts/fdarray-test/master/FDArrayTest65535.otf \
    $gh_url/adobe-fonts/source-han-sans/release/OTC/SourceHanSans-Heavy.ttc \
    $gh_url/adobe-fonts/source-han-sans/release/OTF/SimplifiedChinese/SourceHanSansSC-Regular.otf \
    $gh_url/adobe-fonts/source-han-serif/release/OTC/SourceHanSerif-Regular.ttc \
    $gh_url/adobe-fonts/source-code-pro/release/WOFF2/OTF/SourceCodePro-Medium.otf.woff2 \
    $gh_url/adobe-fonts/source-code-pro/release/WOFF2/VAR/SourceCodeVariable-Italic.ttf.woff2 \
    $gh_url/adobe-fonts/source-sans-pro/release/OTF/SourceSans3-Black.otf \
    $gh_url/adobe-fonts/source-sans-pro/release/WOFF/TTF/SourceSans3-ExtraLight.ttf.woff \
    $gh_url/adobe-fonts/source-sans-pro/release/WOFF/VAR/SourceSans3VF-Roman.ttf.woff \
    $gh_url/adobe-fonts/source-serif/release/TTF/SourceSerif4-LightIt.ttf \
    $gh_url/adobe-fonts/source-serif/release/WOFF/OTF/SourceSerif4Display-Bold.otf.woff \
    $gh_url/adobe-fonts/source-serif/release/WOFF/VAR/SourceSerif4Variable-Italic.otf.woff \
    $gh_url/adobe-fonts/variable-font-collection-test/master/SourceHanVFProto.ttc \
    $gh_url/alif-type/xits/master/XITSMath-Regular.otf \
    $gh_url/googlefonts/noto-emoji/master/fonts/NotoColorEmoji.ttf \
    $gh_url/googlefonts/noto-fonts/master/hinted/ttf/NotoSans/NotoSans-SemiCondensed.ttf \
    $gh_url/weiweihuanghuang/Work-Sans/master/fonts/static/TTF/WorkSans-Regular.ttf \
    https://github.com/adobe-fonts/adobe-variable-font-prototype/releases/download/1.004/AdobeVFPrototype.otf \
    https://github.com/adobe-fonts/adobe-variable-font-prototype/releases/download/1.004/AdobeVFPrototype.ttf \
    https://github.com/adobe-fonts/source-han-super-otc/releases/download/20190603/SourceHanNotoCJK.ttc \
    https://github.com/be5invis/Iosevka/releases/download/v5.0.3/super-ttc-iosevka-5.0.3.zip \
    https://github.com/emojione/emojione-assets/releases/download/3.1.2/emojione-svg.otf \
    https://github.com/mozilla/twemoji-colr/releases/download/v0.5.1/TwemojiMozilla.ttf \
    https://github.com/slavfox/Cozette/releases/download/v.1.9.3/cozette_bitmap.ttf \

do
    font=$(basename $url)
    if [ "$1" == "-f" ] || [ ! -f $font ]; then
        echo "Downloading $font..."
        $download $font $url
        echo ""
    fi
done

unzip -n -q *.zip

echo -e "# Summary:\n"
for suffix in ttf otf ttc woff woff2
do
    echo -e "## ${suffix^^} files:\n"
    du --human-readable *.$suffix
    echo
done

cd $pwd
