all:
	rm -Rf zig/zig-out && rm -Rf zig/zig-cache
	cd zig && zig build -Doptimize=ReleaseFast
	cd ..
	cargo build

test:
	rm -Rf zig/zig-out && rm -Rf zig/zig-cache
	cd zig && zig build -Doptimize=ReleaseFast
	cd ..
	cargo test -- --nocapture
