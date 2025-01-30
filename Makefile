build:
	cargo build-wasi --release

fmt:
	cargo fmt --all $(if $(NO_FIX),-- --check,)

clippy:
	cargo clippy --all --all-targets $(if $(NO_FIX),,--fix) -- -D warnings

licenses-check:
	cargo deny check licenses

bans-check:
	cargo deny check bans

cargo-check:
	cargo check --all --all-targets

test:
	cargo test

node-test-install: build
	cd node-test/ && yarn install

node-test-build: node-test-install
	cd node-test/ && yarn build

node-int-test: node-test-build
	cd node-test/ && yarn node dist/index.js | \
	grep "filename is: $$(readlink -f dist/index.js)" && \
	yarn node dist/index.js |  grep "dirname is: $$(dirname "$$(readlink -f dist/index.js)")" && \
	echo "Success!"	
