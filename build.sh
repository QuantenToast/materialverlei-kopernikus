#!/bin/bash
cd ./frontend
trunk build --release
rm ../api/static/*
cp ./dist/* ./index.css ../api/static/
cd ../api
cargo build --release
