#!/bin/bash

cargo build --target x86_64-pc-windows-gnu --release
if [ -e chinchillas_show_site ]; then
    rm -r chinchillas_show_site
    mkdir chinchillas_show_site
fi

cp -r public_html chinchillas_show_site
cp -r template chinchillas_show_site
cp target/x86_64-pc-windows-gnu/release/chinchillas_show_site.exe chinchillas_show_site/

if [ -e "chinshow.zip" ]; then
    rm chinshow.zip
fi

pushd chinchillas_show_site
zip chinchillas_show_site.zip -r *
mv chinchillas_show_site.zip ../
popd