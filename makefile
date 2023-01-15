prefix=/usr/local

clean:
	cargo clean

test: clean
	cargo test

build: clean test
	cargo build --release

install: build
	install -m 0700 target/release/icepick $(prefix)/bin

uninstall:
	rm $(prefix)/bin/icepick

bench:
	rustup run nightly cargo bench
