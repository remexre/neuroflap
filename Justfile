all:
	cargo check --all
	cargo doc --all
	cargo build --all

distbuild:
	cargo build --release --features=play,simulate,train

play:
	cargo run --release play

test-with-features FEATURES:
	cargo test --no-default-features --features={{FEATURES}}

travis-ci:
	just test-with-features play
	just test-with-features simulate
	just test-with-features train
	just test-with-features play,simulate
	just test-with-features play,train
	just test-with-features simulate,train
	just test-with-features play,simulate,train
	just distbuild

watch TARGET="all":
	watchexec -cre rs,toml "just {{TARGET}}"
