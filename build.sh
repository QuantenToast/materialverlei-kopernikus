#!/bin/bash
cd ./frontend
trunk build --release
cd ../api
cargo build --release
