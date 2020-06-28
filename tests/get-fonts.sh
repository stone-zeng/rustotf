#!/bin/bash

path=./tests/fonts
gh_url=https://raw.githubusercontent.com

for url in \
    $gh_url/googlefonts/noto-fonts/master/hinted/NotoSans/NotoSans-SemiCondensed.ttf \
    $gh_url/adobe-fonts/source-serif-pro/release/TTF/SourceSerifPro-LightIt.ttf      \
    $gh_url/weiweihuanghuang/Work-Sans/master/fonts/static/TTF/WorkSans-Regular.ttf  \

do
    font=$(basename $url)
    echo "Downloading $font..."
    curl -L -o $path/$font $url
    echo ""
done
