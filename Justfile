features := 'adc dma exti flash gpio i2c rtc spi tim uart'
target := `drone print target 2>/dev/null || echo ""`

# Install dependencies
deps:
	type cargo-readme >/dev/null || cargo +stable install cargo-readme
	type drone >/dev/null || cargo install drone
	rustup target add $(drone print target)
	rustup component add clippy
	rustup component add rustfmt

# Reformat the source code
fmt:
	cargo fmt

# Check the source code for mistakes
lint:
	cargo clippy --package drone-stm32-map-svd \
		--target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	cargo clippy --all --exclude drone-stm32-map-svd --features "{{features}}"

# Build the documentation
doc:
	cargo doc --package drone-stm32-map-svd \
		--target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	cargo doc --package drone-stm32-map --features "{{features}}"

# Open the documentation in a browser
doc-open: doc
	cargo doc --package drone-stm32-map --features "{{features}}" --open

# Run the tests
test:
	cargo test --package drone-stm32-map --features "{{features}} std" \
		--target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')

# Test all MCUs
test-all:
	RUSTFLAGS='--cfg cortexm_core="cortexm3_r1p1" --cfg stm32_mcu="stm32f100"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm3_r1p1" --cfg stm32_mcu="stm32f101"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm3_r1p1" --cfg stm32_mcu="stm32f102"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm3_r1p1" --cfg stm32_mcu="stm32f103"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm3_r1p1" --cfg stm32_mcu="stm32f107"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32f303"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32f401"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32f405"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32f407"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32f410"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32f411"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32f412"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32f413"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32f427"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32f429"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32f446"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32f469"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32l4x1"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32l4x2"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32l4x3"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32l4x5"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32l4x6"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32l4r5"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32l4s5"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32l4r7"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32l4s7"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32l4r9"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	RUSTFLAGS='--cfg cortexm_core="cortexm4f_r0p1" --cfg stm32_mcu="stm32l4s9"' cargo test --package drone-stm32-map --features "{{features}} std" --target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')

# Update README.md
readme:
	cargo readme -o README.md

# Bump the versions
version-bump version drone-core-version drone-cortexm-version drone-svd-version:
	sed -i "s/\(api\.drone-os\.com\/drone-stm32-map\/\)[0-9]\+\(\.[0-9]\+\)\+/\1$(echo {{version}} | sed 's/\(.*\)\.[0-9]\+/\1/')/" \
		Cargo.toml src/pieces/*/Cargo.toml src/pieces/Cargo.toml src/periph/*/Cargo.toml svd/Cargo.toml src/lib.rs
	sed -i '/\[.*\]/h;/version = ".*"/{x;s/\[package\]/version = "{{version}}"/;t;x}' \
		Cargo.toml src/pieces/*/Cargo.toml src/pieces/Cargo.toml src/periph/*/Cargo.toml svd/Cargo.toml
	sed -i '/\[.*\]/h;/version = "=.*"/{x;s/\[.*drone-stm32-map-.*\]/version = "={{version}}"/;t;x}' \
		Cargo.toml src/pieces/*/Cargo.toml src/pieces/Cargo.toml src/periph/*/Cargo.toml
	sed -i '/\[.*\]/h;/version = ".*"/{x;s/\[.*drone-core\]/version = "{{drone-core-version}}"/;t;x}' \
		Cargo.toml src/pieces/*/Cargo.toml src/pieces/Cargo.toml src/periph/*/Cargo.toml
	sed -i '/\[.*\]/h;/version = ".*"/{x;s/\[.*drone-cortexm\]/version = "{{drone-cortexm-version}}"/;t;x}' \
		Cargo.toml src/pieces/*/Cargo.toml src/pieces/Cargo.toml src/periph/*/Cargo.toml
	sed -i '/\[.*\]/h;/version = ".*"/{x;s/\[.*drone-svd\]/version = "{{drone-svd-version}}"/;t;x}' \
		svd/Cargo.toml
	sed -i 's/\(drone-stm32-map.*\)version = "[^"]\+"/\1version = "{{version}}"/' \
		src/lib.rs

# Publish to crates.io
publish:
	cd svd && cargo publish \
		--target=$(rustc --version --verbose | sed -n '/host/{s/.*: //;p}')
	sleep 30
	cd src/pieces/1 && cargo publish
	cd src/pieces/2 && cargo publish
	cd src/pieces/3 && cargo publish
	cd src/pieces/4 && cargo publish
	cd src/pieces/5 && cargo publish
	cd src/pieces/6 && cargo publish
	cd src/pieces/7 && cargo publish
	cd src/pieces/8 && cargo publish
	cd src/pieces/9 && cargo publish
	cd src/pieces/10 && cargo publish
	cd src/pieces/11 && cargo publish
	cd src/pieces/12 && cargo publish
	sleep 30
	cd src/pieces && cargo publish
	sleep 30
	cd src/periph/adc && cargo publish
	cd src/periph/dma && cargo publish
	cd src/periph/exti && cargo publish
	cd src/periph/flash && cargo publish
	cd src/periph/gpio && cargo publish
	cd src/periph/i2c && cargo publish
	cd src/periph/rtc && cargo publish
	cd src/periph/spi && cargo publish
	cd src/periph/tim && cargo publish
	cd src/periph/uart && cargo publish
	sleep 30
	cargo publish --features "{{features}}"

# Publish the docs to api.drone-os.com
publish-doc: doc
	dir=$(sed -n 's/.*api\.drone-os\.com\/\(.*\/.*\)\/.*\/"/\1/;T;p' Cargo.toml) \
		&& rm -rf ../drone-api/$dir \
		&& cp -rT target/doc ../drone-api/$dir \
		&& cp -rT target/{{target}}/doc ../drone-api/$dir \
		&& echo '<!DOCTYPE html><meta http-equiv="refresh" content="0; URL=./drone_stm32_map">' > ../drone-api/$dir/index.html \
		&& cd ../drone-api && git add $dir && git commit -m "Docs for $dir"
