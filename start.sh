#!/bin/bash
cd ./frontend
trunk build --release
rm ../api/static/*
mv dist/* index.css ../api/static/
cd ../api
screen -dmS mat-serve cargo run --release
