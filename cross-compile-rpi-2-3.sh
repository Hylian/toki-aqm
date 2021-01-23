#!/bin/sh -e

export RPXC_IMAGE=rpxc-bin
docker build -t rpxc-bin .
docker run rpxc-bin > ./rpxc-bin
chmod +x rpxc-bin
./rpxc-bin cargo build --release --target=armv7-unknown-linux-gnueabihf
rm rpxc-bin
