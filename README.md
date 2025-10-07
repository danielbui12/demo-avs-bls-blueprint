# How to run

```sh
# Build the blueprint
cargo build --release

# Deploy the blueprint
## First, deploy the TangleTaskManager
## Then, deploy the TangleServiceManager
cargo tangle blueprint deploy eigenlayer \
  --devnet \
  --ordered-deployment

# Using cast to call initialize in TaskManager
cast send 0xc0f115a19107322cfbf1cdbc7ea011c19ebdb4f8 "initialize(address,address,address,address,address,address)" \
  0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC \
  0x90F79bf6EB2c4f870365E785982E1f101E93b906 \
  0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65 \
  0x8a791620dd6260079bf849dc5567adc3f2fdc318 \
  0x12699471dF8dca329C76D72823B1b79d55709384 \
  0xc96304e3c037f81da488ed9dea1d8f2a48278a75 \
  --private-key 0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6

# Open another terminal
RUST_LOG=all cargo tangle blueprint run \
     -p eigenlayer \
     -u http://localhost:55003/ \
     --keystore-path ./test-keystore
```