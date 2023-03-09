run:
	cargo run --features bevy/dynamic_linking

build:
	cargo build --release
	
build_win:
		cargo build --release --target x86_64-pc-windows-gnu

.PHONY:
	run build build_win