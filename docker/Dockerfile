# SPDX-License-Identifier: LGPL-3.0-or-later
#
# (c) SYSTEC electronic AG, D-08468 Heinsdorfergrund, Am Windrad 2
#     www.systec-electronic.com

FROM rust:1.75-slim

RUN    dpkg --add-architecture armhf \
    && dpkg --add-architecture arm64 \
    && apt-get update \
    && apt-get install -y \
        libiio-dev:armhf \
        libsensors4-dev:armhf \
        gcc-arm-linux-gnueabihf \
    && apt-get install -y \
        libiio-dev:arm64 \
        libsensors4-dev:arm64 \
        gcc-aarch64-linux-gnu \
    && apt-get install -y \
        libiio-dev \
        libsensors4-dev

RUN rustup target add \
        armv7-unknown-linux-gnueabihf \
        aarch64-unknown-linux-gnu \
    && rustup component add rustfmt clippy \
    && cargo install cbindgen@0.28.0 --locked \
    && cargo install cargo-deb@2.9.3 --locked
