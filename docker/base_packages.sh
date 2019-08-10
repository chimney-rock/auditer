#!/usr/bin/env bash

apt-get update
apt-get --no-install-recommends -y install \
  ca-certificates \
  libssl-dev \
  musl-tools \
  musl-dev \
  pkg-config

rustup target add ${CARGO_BUILD_TARGET}

# Install dumb-init as our process supervisor
wget --quiet -O /dumb-init https://github.com/Yelp/dumb-init/releases/download/v1.2.2/dumb-init_1.2.2_amd64
chmod +x /dumb-init
