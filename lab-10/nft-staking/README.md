# Anchor Program: Quick Clean & Rebuild

## 1. Clean everything

```bash
# Clean Anchor and Rust build artifacts
anchor clean
cargo clean

# Clean frontend dependencies
rm -rf node_modules
rm -f package-lock.json   # optional, if using npm
rm -f yarn.lock           # optional, if using yarn
```

## 2. Rebuild

```bash
npm install
yarn install
anchor build
```

restart rust-analyzer server

## 3. Test with LiteSVM

```bash
solana program dump -u m metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metaplex_token_metadata_program.so
```

