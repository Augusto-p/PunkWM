#!/bin/bash

path=$(pwd)

cd "$path/punkwm_dock/src-tauri"
cargo build
cd "$path/punkwm_lock/src-tauri"
cargo build
cd "$path"
cargo build
mv "$path/punkwm_dock/src-tauri/target/debug/punkwm_dock" "$path/dock"
mv "$path/punkwm_lock/src-tauri/target/debug/punkwm_lock" "$path/lock"
mv "$path/src-tauri/target/debug/punkwm" "$path/punkwm"
killall Xorg
