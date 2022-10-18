#!/bin/bash

cargo build --release
if [ -e chinchillas_show_site-linux ]; then
    rm -r chinchillas_show_site-linux
fi

mkdir chinchillas_show_site-linux
cp -r public_html chinchillas_show_site-linux
cp -r template chinchillas_show_site-linux
cp Rocket.toml chinchillas_show_site-linux
cp show_info.toml chinchillas_show_site-linux
cp target/release/chinchillas_show_site chinchillas_show_site-linux/

if [ -e chinchillas_show_site.tar.gz ]; then
    rm chinchillas_show_site.tar.gz
fi

pushd chinchillas_show_site-linux
zip chinchillas_show_site.tar.gz -r *
mv chinchillas_show_site.tar.gz ../
popd
