#!/bin/bash

# Prerequisites:
#     sudo apt install inkscape icoutils

cd "$(dirname "$(readlink -f "${0}")")"

# SVG to PNG
PNGLIST=''
for SIZE in 16 32 48
do
  inkscape -z ../img/levidrone-icon.svg --export-png=favicon${SIZE}.png --export-width=$SIZE
  PNGLIST="$PNGLIST favicon${SIZE}.png"
done


# PNG to ICO
icotool -o ../static/img/favicon.ico -c $PNGLIST

# Clean up
rm -rf $PNGLIST

# PNG icons
for SIZE in 16 32 180 192 512
do
  inkscape -z ../img/levidrone-icon.svg --export-png=../static/img/icon${SIZE}.png --export-width=$SIZE
done
