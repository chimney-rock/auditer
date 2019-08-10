#!/usr/bin/env bash

cargo build --release --features vendored
cp target/${CARGO_BUILD_TARGET}/release/auditer ${INSTALL_PATH}/
strip ${INSTALL_PATH}/auditer
