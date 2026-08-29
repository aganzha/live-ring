#!/bin/bash
clear 
cargo build --lib --release 
export RUST_BACKTRACE=1
export NO_ALPHA=1
emacs

