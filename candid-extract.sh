#!/bin/bash
cargo build --target wasm32-unknown-unknown --release -p backend --locked
candid-extractor target/wasm32-unknown-unknown/release/backend.wasm > .candid/backend.did
