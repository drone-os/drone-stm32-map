#!/bin/bash

source $(dirname $0)/_env.sh
set -x

cargo doc \
  --package drone-stm32-map-svd
cargo doc --target $BUILD_TARGET --features "$SELECTED_FEATURE" \
  --all --exclude drone-stm32-map-svd
