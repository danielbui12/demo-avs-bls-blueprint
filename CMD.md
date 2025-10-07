function initialize(
  address initialOwner,  // anvil account #2
  address _aggregator,   // anvil account #3
  address _generator,    // anvil account #4
  address _allocationManager,
  address _slasher,
  address _serviceManager
)

cast send 0xc0f115a19107322cfbf1cdbc7ea011c19ebdb4f8 "initialize(address,address,address,address,address,address)" \
  0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC \
  0x90F79bf6EB2c4f870365E785982E1f101E93b906 \
  0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65 \
  0x8a791620dd6260079bf849dc5567adc3f2fdc318 \
  0x12699471dF8dca329C76D72823B1b79d55709384 \
  0xc96304e3c037f81da488ed9dea1d8f2a48278a75 \
  --private-key 0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6

cast call 0xec4cfde48eadca2bc63e94bb437bbeace1371bf3 "underlyingToken()(address)"

cast call 0x4c5859f0f772848b2d91f1d83e2fe57935348029 "name()(string)"

cast call 0x4c5859f0f772848b2d91f1d83e2fe57935348029 "balanceOf(address)(uint256)" 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266


cast send 0x8a791620dd6260079bf849dc5567adc3f2fdc318 "updateAVSMetadataURI(address, string)" 0xc96304e3c037f81da488ed9dea1d8f2a48278a75  "https://www.google.com" --private-key 0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6