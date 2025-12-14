#!/bin/bash

SRC="src-tauri/icons/icon.png"
DST="src-tauri/icons"

# PNGs
magick "$SRC" -resize 32x32 "$DST/32x32.png"
magick "$SRC" -resize 128x128 "$DST/128x128.png"
magick "$SRC" -resize 256x256 "$DST/128x128@2x.png"

# ICNS (macOS)
magick "$SRC" -resize 16x16 "$DST/icon-16.png"
magick "$SRC" -resize 32x32 "$DST/icon-32.png"
magick "$SRC" -resize 64x64 "$DST/icon-64.png"
magick "$SRC" -resize 128x128 "$DST/icon-128.png"
magick "$SRC" -resize 256x256 "$DST/icon-256.png"
magick "$SRC" -resize 512x512 "$DST/icon-512.png"
magick "$DST/icon-16.png" "$DST/icon-32.png" "$DST/icon-64.png" "$DST/icon-128.png" "$DST/icon-256.png" "$DST/icon-512.png" "$DST/icon.icns"

# ICO (Windows)
magick "$SRC" -resize 16x16 "$DST/icon-16.ico"
magick "$SRC" -resize 32x32 "$DST/icon-32.ico"
magick "$SRC" -resize 48x48 "$DST/icon-48.ico"
magick "$SRC" -resize 64x64 "$DST/icon-64.ico"
magick "$SRC" -resize 128x128 "$DST/icon-128.ico"
magick "$SRC" -resize 256x256 "$DST/icon-256.ico"
magick "$SRC" -resize 512x512 "$DST/icon-512.ico"
magick "$DST/icon-16.ico" "$DST/icon-32.ico" "$DST/icon-48.ico" "$DST/icon-64.ico" "$DST/icon-128.ico" "$DST/icon-256.ico" "$DST/icon-512.ico" "$DST/icon.ico"

# Cleanup temp icon PNGs/ICOs if you want
rm "$DST"/icon-*.png "$DST"/icon-*.ico

echo "All icons generated in $DST"