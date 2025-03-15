release:
	cargo build --release

debug:
	cargo build

test:
	cargo test

test-ignored:
	cargo test -- --ignored

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

.PHONY: release debug test test-ignored bench publish-dry-run publish run clean