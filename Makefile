all: test

build:
	@cargo build --all-features

doc:
	@cargo doc --all-features

test:
	@cargo insta test --review

reset-db:
	cd tests/data && ./reload_db.sh

# update-sanpshots:
# 	TRYCMD=overwrite cargo test --test cli_snapshot_testing

# sanpshot-testing: update-sanpshots
# TRYCMD=dump cargo test --test cli_snapshot_testing

cargotest:
	@echo "CARGO TESTS"
	@rustup component add rustfmt 2> /dev/null
	@cargo test

format:
	@rustup component add rustfmt 2> /dev/null
	@cargo fmt --all

toolchain:
	rustup toolchain install nightly
	rustup component add rust-src --toolchain nightly

release-apple:
	cargo +nightly build -Z build-std=std,panic_abort --target x86_64-apple-darwin --release

release-apple-m1:
	cargo +nightly build -Z build-std=std,panic_abort --target aarch64-apple-darwin --release

format-check:
	@rustup component add rustfmt 2> /dev/null
	@cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	@cargo clippy

.PHONY: all doc test cargotest format format-check lint update-readme
