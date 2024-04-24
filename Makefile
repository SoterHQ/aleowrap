.PHONY: mainnetv0

mainnetv0:
	cargo build --release --features mainnetv0

.PHONY: testnetv0

testnetv0:
	cargo build --release --features testnetv0