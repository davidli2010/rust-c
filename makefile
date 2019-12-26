GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)
run: clean build
	LD_LIBRARY_PATH=./target/debug ./c_src/main
clean:
	$(CARGO_BIN) clean
	rm -f ./c_src/main
build:
	$(CARGO_BIN) build
	$(GCC_BIN) -g -o ./c_src/main ./c_src/main.c -Isrc -L ./target/debug -lrustc