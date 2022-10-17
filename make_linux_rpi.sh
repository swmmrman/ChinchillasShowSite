#!/bin/bash

cross build --release --target=armv7-unknown-linux-gnueabihf
if [ -e chinchillas_show_site-linux-rpi ]; then
    rm -r chinchillas_show_site-linux-rpi
fi

mkdir chinchillas_show_site-linux-rpi
cp -r public_html chinchillas_show_site-linux-rpi
cp -r template chinchillas_show_site-linux-rpi
cp target/release/chinchillas_show_site chinchillas_show_site-linux-rpi/

if [ -e chinchillas_show_site-rpi.tar.gz ]; then
    rm chinchillas_show_site-rpi.tar.gz
fi

pushd chinchillas_show_site-linux-rpi
zip chinchillas_show_site-rpi.tar.gz -r *
mv chinchillas_show_site-rpi.tar.gz ../
popd
