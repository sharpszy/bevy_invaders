run:
	#cargo run --release --features bevy/dynamic
	cargo run --release

build:
	cargo build --release
	
build_win:
		cargo build --release --target x86_64-pc-windows-gnu

.PHONY:
	build build_win