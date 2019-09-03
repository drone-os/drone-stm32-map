build_target := 'thumbv7em-none-eabihf'
features := 'stm32l4s7'

# Install dependencies
deps:
	rustup target add {{build_target}}
	rustup component add clippy
	rustup component add rustfmt
	rustup component add rls rust-analysis rust-src
	type cargo-readme >/dev/null || cargo +stable install cargo-readme

# Reformat the source code
fmt:
	cargo fmt

# Check for mistakes
lint:
	cargo clippy --package drone-stm32-map-svd
	cargo clippy --target {{build_target}} --features "{{features}}" --all --exclude drone-stm32-map-svd

# Check each feature
check-all:
	cargo check --package drone-stm32-map --features stm32f100 --target thumbv7m-none-eabi
	cargo check --package drone-stm32-map --features stm32f101 --target thumbv7m-none-eabi
	cargo check --package drone-stm32-map --features stm32f102 --target thumbv7m-none-eabi
	cargo check --package drone-stm32-map --features stm32f103 --target thumbv7m-none-eabi
	cargo check --package drone-stm32-map --features stm32f107 --target thumbv7m-none-eabi
	cargo check --package drone-stm32-map --features stm32l4x1 --target thumbv7em-none-eabihf
	cargo check --package drone-stm32-map --features stm32l4x2 --target thumbv7em-none-eabihf
	cargo check --package drone-stm32-map --features stm32l4x3 --target thumbv7em-none-eabihf
	cargo check --package drone-stm32-map --features stm32l4x5 --target thumbv7em-none-eabihf
	cargo check --package drone-stm32-map --features stm32l4x6 --target thumbv7em-none-eabihf
	cargo check --package drone-stm32-map --features stm32l4r5 --target thumbv7em-none-eabihf
	cargo check --package drone-stm32-map --features stm32l4s5 --target thumbv7em-none-eabihf
	cargo check --package drone-stm32-map --features stm32l4r7 --target thumbv7em-none-eabihf
	cargo check --package drone-stm32-map --features stm32l4s7 --target thumbv7em-none-eabihf
	cargo check --package drone-stm32-map --features stm32l4r9 --target thumbv7em-none-eabihf
	cargo check --package drone-stm32-map --features stm32l4s9 --target thumbv7em-none-eabihf

# Generate the docs
doc:
	cargo doc --package drone-stm32-map-svd
	cargo doc --target {{build_target}} --features "{{features}}" --package drone-stm32-map

# Open the docs in a browser
doc-open: doc
	cargo doc --target {{build_target}} --features "{{features}}" --package drone-stm32-map --open

# Update README.md
readme:
	cargo readme -o README.md

# Publish to crates.io
publish:
	cd svd && cargo publish
	cd src/pieces/1 && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/pieces/2 && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/pieces/3 && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/pieces/4 && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/pieces/5 && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/pieces/6 && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/pieces/7 && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/pieces/8 && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/pieces/9 && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/pieces/10 && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/pieces/11 && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/pieces/12 && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/pieces && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/periph/adc && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/periph/dma && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/periph/exti && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/periph/gpio && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/periph/i2c && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/periph/rtc && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/periph/spi && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/periph/tim && cargo publish --target {{build_target}} --features "{{features}}"
	cd src/periph/uart && cargo publish --target {{build_target}} --features "{{features}}"
	cargo publish --target {{build_target}} --features "{{features}}"
