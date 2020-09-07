#!/bin/bash

path=./tests/fonts
gh_url=https://raw.githubusercontent.com

for url in \
    $gh_url/googlefonts/noto-fonts/master/hinted/NotoSans/NotoSans-SemiCondensed.ttf                     \
    $gh_url/adobe-fonts/source-serif-pro/release/TTF/SourceSerifPro-LightIt.ttf                          \
    $gh_url/weiweihuanghuang/Work-Sans/master/fonts/static/TTF/WorkSans-Regular.ttf                      \
    $gh_url/adobe-fonts/source-han-sans/release/OTF/SimplifiedChinese/SourceHanSansSC-Regular.otf        \
    $gh_url/adobe-fonts/source-sans-pro/release/OTF/SourceSansPro-Black.otf                              \
    $gh_url/alif-type/xits/master/XITSMath-Regular.otf                                                   \
    $gh_url/adobe-fonts/source-han-sans/release/OTC/SourceHanSans-Heavy.ttc                              \
    $gh_url/adobe-fonts/source-han-serif/release/OTC/SourceHanSerif-Regular.ttc                          \
    https://github.com/adobe-fonts/source-han-super-otc/releases/download/20190603/SourceHanNotoCJK.ttc  \
    $gh_url/adobe-fonts/source-sans-pro/release/WOFF/TTF/SourceSans3-ExtraLight.ttf.woff                 \
    $gh_url/adobe-fonts/source-sans-pro/release/WOFF/VAR/SourceSans3VF-Roman.ttf.woff                    \
    $gh_url/adobe-fonts/source-serif-pro/release/WOFF/OTF/SourceSerifPro-Bold.otf.woff                   \
    $gh_url/adobe-fonts/source-serif-pro/release/WOFF/VAR/SourceSerifVariable-Italic.otf.woff            \

do
    font=$(basename $url)
    echo "Downloading $font..."
    curl -L -o $path/$font $url
    echo ""
done
