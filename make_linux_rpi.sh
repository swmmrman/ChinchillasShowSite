#!/bin/bash

cross build --release --target=aarch64-unknown-linux-gnu
if [ -e chinchillas_show_site-linux-rpi ]; then
    rm -r chinchillas_show_site-linux-rpi
fi

mkdir chinchillas_show_site-linux-rpi
cp -r public_html chinchillas_show_site-linux-rpi
cp -r template chinchillas_show_site-linux-rpi
cp Rocket.toml chinchillas_show_site-linux-rpi
cp show_info.toml chinchillas_show_site-linux-rpi
cp target/aarch64-unknown-linux-gnu/release/chinchillas_show_site chinchillas_show_site-linux-rpi/

if [ -e chinchillas_show_site-rpi.tar.gz ]; then
    rm chinchillas_show_site-rpi.tar.gz
fi

tar -czf chinchillas_show_site-rpi.tar.gz chinchillas_show_site-linux-rpi/
