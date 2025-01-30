build:
	$(if $(NO_BUILD),echo "skipping cargo build",cargo build --target wasm32-wasip1 --release)

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

# Note, we use case insensitive grep since windows capitalizes its drives but readlink does not
node-int-test: node-test-build
	cd node-test/ && echo -e "Testing output:\n$$(yarn node dist/index.js)"
	cd node-test/ && echo -e "For values:\nfilename is $$(readlink -f dist/index.js)\ndirname is $$(dirname "$$(readlink -f dist/index.js)")\n"
	cd node-test/ && yarn node dist/index.js | \
	grep -i "filename is $$(readlink -f dist/index.js)" && \
	yarn node dist/index.js | grep -i "dirname is $$(dirname "$$(readlink -f dist/index.js)")" && \
	echo "Success!"
