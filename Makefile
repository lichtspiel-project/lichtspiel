release:
	cargo build --release

debug:
	cargo build

test:
	cargo test

bench:
	cargo bench

publish-dry-run:
	cargo publish --dry-run

publish:
	cargo publish

run:
	cargo run

clean:
	rm -rf target/*

.PHONY: release debug test bench publish-dry-run publish run clean