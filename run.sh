#!/bin/bash
clear 
export RUST_BACKTRACE=1
export NO_ALPHA=1
cargo build --lib --release && emacs

