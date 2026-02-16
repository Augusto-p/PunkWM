#!/bin/bash

path=$(pwd)

cd "$path/punkwm_dock/src-tauri"
cargo build
cd "$path"
cargo build
mv "$path/punkwm_dock/src-tauri/target/debug/punkwm_dock" "$path/dock"
killall Xorg
