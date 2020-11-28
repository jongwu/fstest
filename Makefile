all:
	cargo build
	cp target/debug/fstest .

clean:
	rm -f fstest target/debug/fstest
