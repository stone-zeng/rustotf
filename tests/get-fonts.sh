#!/bin/bash

path=./tests/fonts
gh_url=https://raw.githubusercontent.com

for url in \
    $gh_url/adobe-fonts/cjk-radicals/master/CJKRadicals-Regular.otf                                      \
    $gh_url/adobe-fonts/source-han-sans/release/OTC/SourceHanSans-Heavy.ttc                              \
    $gh_url/adobe-fonts/source-han-sans/release/OTF/SimplifiedChinese/SourceHanSansSC-Regular.otf        \
    $gh_url/adobe-fonts/source-han-serif/release/OTC/SourceHanSerif-Regular.ttc                          \
    $gh_url/adobe-fonts/source-sans-pro/release/OTF/SourceSans3-Black.otf                                \
    $gh_url/adobe-fonts/source-sans-pro/release/WOFF/TTF/SourceSans3-ExtraLight.ttf.woff                 \
    $gh_url/adobe-fonts/source-sans-pro/release/WOFF/VAR/SourceSans3VF-Roman.ttf.woff                    \
    $gh_url/adobe-fonts/source-serif-pro/release/TTF/SourceSerifPro-LightIt.ttf                          \
    $gh_url/adobe-fonts/source-serif-pro/release/WOFF/OTF/SourceSerifPro-Bold.otf.woff                   \
    $gh_url/adobe-fonts/source-serif-pro/release/WOFF/VAR/SourceSerifVariable-Italic.otf.woff            \
    $gh_url/alif-type/xits/master/XITSMath-Regular.otf                                                   \
    $gh_url/googlefonts/noto-fonts/master/hinted/ttf/NotoSans/NotoSans-SemiCondensed.ttf                 \
    $gh_url/weiweihuanghuang/Work-Sans/master/fonts/static/TTF/WorkSans-Regular.ttf                      \
    https://github.com/adobe-fonts/source-han-super-otc/releases/download/20190603/SourceHanNotoCJK.ttc  \
    https://github.com/emojione/emojione-assets/releases/download/3.1.2/emojione-svg.otf                 \
    https://github.com/slavfox/Cozette/releases/download/v.1.9.3/cozette_bitmap.ttf                      \

do
    font=$(basename $url)
    echo "Downloading $font..."
    curl -L -o $path/$font $url
    echo ""
done
