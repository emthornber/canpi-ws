################################################################################
#
#   2 May, 2024 - E M Thornber
#   Created
#
################################################################################

export SDIR := ${shell pwd}
export BFILE := "$(SDIR)/target/arm-unknown-linux-gnueabihf/release/canpi-ws"
export ODIR := "$(SDIR)/package"

all: clean package

.PHONY: all build clean config release run test

build:
	cargo build

clean:
	cargo clean

package: config release
	$(MAKE) -f $@/Makefile

config:
	$(MAKE) -C $@

release:
	cargo build --release

run:
	cargo run

test:
	cargo test
