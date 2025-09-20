#!/bin/bash
# Cleans Anchor, Rust, and frontend dependencies

echo "Cleaning Anchor & Rust build artifacts..."
anchor clean >/dev/null 2>&1
cargo clean >/dev/null 2>&1

echo "Cleaning frontend dependencies..."
rm -rf node_modules >/dev/null 2>&1
rm -rf test-ledger >/dev/null 2>&1
rm -f package-lock.json yarn.lock Cargo.lock >/dev/null 2>&1

echo "Clean complete ✅"
