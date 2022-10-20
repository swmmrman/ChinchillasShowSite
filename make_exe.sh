#!/bin/bash

cross build --target x86_64-pc-windows-gnu --release
if [ -e chinchillas_show_site-windows ]; then
    rm -r chinchillas_show_site-windows
fi

mkdir chinchillas_show_site-windows
cp -r public_html chinchillas_show_site-windows
cp -r template chinchillas_show_site-windows
cp Rocket.toml chinchillas_show_site-windows
cp show_info.toml chinchillas_show_site-windows
cp target/x86_64-pc-windows-gnu/release/chinchillas_show_site.exe chinchillas_show_site-windows/

if [ -e "chinshow.zip" ]; then
    rm chinshow.zip
fi

pushd chinchillas_show_site-windows
zip chinchillas_show_site.zip -r *
mv chinchillas_show_site.zip ../
popd