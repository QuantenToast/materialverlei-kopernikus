#!/bin/bash
cd ./frontend
trunk build --release
mv dist/* ../api/static/
cd ../api
screen -dmS mat-serve cargo run --release
