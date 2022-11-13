{
  description = "STM32 peripheral mappings for Drone, an Embedded Operating System";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-22.05";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, utils, nixpkgs, fenix }:
    utils.lib.eachDefaultSystem (system:
      let
        buildTarget = "thumbv7em-none-eabihf";
        rustFlags = ''--cfg drone_cortexm="cortexm4f_r0p1" --cfg drone_stm32_map="stm32l4s9"'';
        rustChannel = {
          channel = "nightly";
          date = "2022-11-12";
          sha256 = "NZrKSshDgITZuDSffP89NpZl/pQlblc7arXatkV+O9A=";
        };

        pkgs = nixpkgs.legacyPackages.${system};
        rustToolchain = with fenix.packages.${system}; combine
          ((with toolchainOf rustChannel; [
            rustc
            cargo
            clippy
            rustfmt
            rust-src
          ]) ++ (with targets.${buildTarget}.toolchainOf rustChannel; [
            rust-std
          ]));
        rustAnalyzer = fenix.packages.${system}.rust-analyzer;

        crossEnv = {
          CARGO_BUILD_TARGET = buildTarget;
        };
        nativeEnv = {
          CARGO_BUILD_TARGET = pkgs.stdenv.targetPlatform.config;
        };

        cargoRdme = (
          pkgs.rustPlatform.buildRustPackage rec {
            name = "cargo-rdme";
            src = pkgs.fetchFromGitHub {
              owner = "orium";
              repo = name;
              rev = "v0.7.3";
              sha256 = "qzit/uYkyWiOqpO5sHYo2hKJvOhovcO+oVbq/Bo2HsI=";
            };
            cargoSha256 = "lbyLVmSLNt4mt6hQbJnCuNL1Y1/2E/81sVpLYOkv7w8=";
            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ pkgs.openssl ];
            doCheck = false;
          });

        checkAll = pkgs.writeShellScriptBin "check-all" ''
          set -ex
          cargo rdme --check
          cargo fmt --all --check
          cargo clippy --workspace --exclude drone-stm32-map-svd --features all -- --deny warnings
          nix develop '.#native' -c cargo clippy --package drone-stm32-map-svd -- --deny warnings
          if [ $1 = "one" ]; then
            nix develop '.#native' -c cargo test --package drone-stm32-map --features all,std
          else
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm3_r1p1\" --cfg drone_stm32_map=\"stm32f100\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm3_r1p1\" --cfg drone_stm32_map=\"stm32f101\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm3_r1p1\" --cfg drone_stm32_map=\"stm32f102\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm3_r1p1\" --cfg drone_stm32_map=\"stm32f103\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm3_r1p1\" --cfg drone_stm32_map=\"stm32f107\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32f303\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32f401\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32f405\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32f407\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32f410\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32f411\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32f412\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32f413\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32f427\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32f429\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32f446\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32f469\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32l4x1\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32l4x2\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32l4x3\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32l4x5\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32l4x6\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32l4r5\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32l4s5\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32l4r7\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32l4s7\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32l4r9\"" cargo test --package drone-stm32-map --features all,std'
            nix develop '.#native' -c sh -c 'CARGO_BUILD_RUSTFLAGS="--cfg drone_cortexm=\"cortexm4f_r0p1\" --cfg drone_stm32_map=\"stm32l4s9\"" cargo test --package drone-stm32-map --features all,std'
          fi
          RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --package drone-stm32-map --features all
          RUSTDOCFLAGS='-D warnings' nix develop '.#native' -c cargo doc --no-deps --package drone-stm32-map-svd
        '';

        updateVersions = pkgs.writeShellScriptBin "update-versions" ''
          sed -i "s/\(api\.drone-os\.com\/drone-stm32-map\/\)[0-9]\+\(\.[0-9]\+\)\+/\1$(echo $1 | sed 's/\(.*\)\.[0-9]\+/\1/')/" \
            Cargo.toml src/pieces/*/Cargo.toml src/pieces/Cargo.toml src/periph/*/Cargo.toml svd/Cargo.toml src/lib.rs
          sed -i "/\[.*\]/h;/version = \".*\"/{x;s/\[workspace.package\]/version = \"$1\"/;t;x}" \
            Cargo.toml
          sed -i "/\[.*\]/h;/version = \"=.*\"/{x;s/\[.*drone-stm32-map-.*\]/version = \"=$1\"/;t;x}" \
            Cargo.toml
          sed -i "/\[.*\]/h;/version = \".*\"/{x;s/\[.*drone-cortexm\]/version = \"$2\"/;t;x}" \
            Cargo.toml
          sed -i "/\[.*\]/h;/version = \".*\"/{x;s/\[.*drone-core\]/version = \"$3\"/;t;x}" \
            Cargo.toml
          sed -i "/\[.*\]/h;/version = \".*\"/{x;s/\[.*drone-config\]/version = \"$4\"/;t;x}" \
            Cargo.toml
          sed -i "/\[.*\]/h;/version = \".*\"/{x;s/\[.*drone-svd\]/version = \"$5\"/;t;x}" \
            Cargo.toml
          sed -i "s/\(drone-stm32-map.*\)version = \"[^\"]\+\"/\1version = \"$1\"/" \
            src/lib.rs
        '';

        publishCrates = pkgs.writeShellScriptBin "publish-crates" ''
          cd svd && nix develop '.#native' -c cargo publish
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
          cd src/periph/gpio && cargo publish
          cd src/periph/i2c && cargo publish
          cd src/periph/rtc && cargo publish
          cd src/periph/spi && cargo publish
          cd src/periph/tim && cargo publish
          cd src/periph/uart && cargo publish
          sleep 30
          cargo publish --features all
        '';

        publishDocs = pkgs.writeShellScriptBin "publish-docs" ''
          dir=$(sed -n 's/.*api\.drone-os\.com\/\(.*\/.*\)\/.*\/"/\1/;T;p' Cargo.toml) \
            && rm -rf ../drone-api/$dir \
            && cp -rT target/doc ../drone-api/$dir \
            && cp -rT target/$CARGO_BUILD_TARGET/doc ../drone-api/$dir \
            && echo '<!DOCTYPE html><meta http-equiv="refresh" content="0; URL=./drone_stm32_map">' > ../drone-api/$dir/index.html \
            && cd ../drone-api && git add $dir && git commit -m "Docs for $dir"
        '';

        mkShell = extraEnv: pkgs.mkShell ({
          nativeBuildInputs = [
            rustToolchain
            rustAnalyzer
            cargoRdme
            checkAll
            updateVersions
            publishCrates
            publishDocs
          ];
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          CARGO_BUILD_RUSTFLAGS = rustFlags;
        } // extraEnv);
      in
      {
        devShells = rec {
          cross = mkShell (crossEnv // { name = "cross"; });
          native = mkShell (nativeEnv // { name = "native"; });
          default = cross;
        };
      }
    );
}
