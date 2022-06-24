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
        target = "thumbv7em-none-eabihf";
        cortexm_core = "cortexm4f_r0p1";
        stm32_mcu = "stm32l4s9";
        pkgs = nixpkgs.legacyPackages.${system};
        rustChannel = {
          channel = "nightly";
          date = "2022-06-18";
          sha256 = "TX82NKIM6/V8rJ8CskbwizaDCvQeF0KvN3GkcY4XQzQ=";
        };
        rustToolchain = with fenix.packages.${system};
          let toolchain = toolchainOf rustChannel; in
          combine [
            toolchain.rustc
            toolchain.cargo
            toolchain.clippy
            toolchain.rust-src
            (targets.${target}.toolchainOf rustChannel).rust-std
          ];
        rustFmt = (fenix.packages.${system}.toolchainOf rustChannel).rustfmt;
        rustAnalyzer = fenix.packages.${system}.rust-analyzer;
        crossEnv = {
          CARGO_BUILD_TARGET = target;
        };
        nativeEnv = {
          CARGO_BUILD_TARGET = pkgs.stdenv.targetPlatform.config;
        };
        mkShell = extraEnv: pkgs.mkShell ({
          nativeBuildInputs = [
            rustToolchain
            rustFmt
            rustAnalyzer
          ];
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          CARGO_BUILD_RUSTFLAGS = ''--cfg cortexm_core="${cortexm_core}" --cfg stm32_mcu="${stm32_mcu}"'';
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
