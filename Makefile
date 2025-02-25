release:
	cargo build --release

test:
	cargo test

bench:
	cargo bench

publish-dry:
	cargo publish --dry-run

publish:
	cargo publish

run:
	cargo run