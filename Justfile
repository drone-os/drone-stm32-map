build_target := 'thumbv7em-none-eabihf'
features := 'stm32l4s7'

# Check with clippy.
clippy:
	cargo clippy \
		--package drone-stm32-map-svd
	cargo clippy --target {{build_target}} --features {{features}} \
		--all --exclude drone-stm32-map-svd

# Generate documentation.
doc:
	cargo doc \
		--package drone-stm32-map-svd
	cargo doc --target {{build_target}} --features {{features}} \
		--all --exclude drone-stm32-map-svd

# Generate README.md from src/lib.rs.
readme:
	cargo readme -o README.md
