SHELL := /bin/bash

export OPENSSL_LIB_DIR := ${PWD}/tmp/openssl-1.1.1i/
export OPENSSL_INCLUDE_DIR := ${PWD}/tmp/openssl-1.1.1i/include

build:
	chmod +x openssl_build
	cargo build --target x86_64-pc-windows-gnu --release
	cargo build --release
	./openssl_build
	cargo build --target armv7-unknown-linux-gnueabihf --release
build_pi:
	chmod +x openssl_build
	./openssl_build
	cargo build --target armv7-unknown-linux-gnueabihf --release
deploy_pi:
	scp target/armv7-unknown-linux-gnueabihf/release/stock-fetcher pi@${ip}:~/./