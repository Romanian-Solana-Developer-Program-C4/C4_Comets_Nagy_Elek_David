#!/bin/bash
# Rebuilds Anchor program and frontend dependencies

echo "Installing frontend dependencies..."
npm install || yarn install

echo "Building Anchor program..."
anchor build

echo "Build complete ✅"
