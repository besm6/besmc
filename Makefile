#
# make
# make all      -- build everything
#
# make test     -- run unit tests
#
# make install  -- install binaries to ~/.cargo/bin/
#
# make clean    -- remove build files
#

all:
	cargo build

run:
	cargo run

test:
	cargo test

install:
	cargo install --path .

clean:
	rm -rf build
