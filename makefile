GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)
run: build
	./c_src/main
clean:
	$(CARGO_BIN) clean
	rm -f ./c_src/main
	rm -f ./c_src/rust-c.h
	rm -f ./src/bindings.rs
build:
	$(CARGO_BIN) build
	$(GCC_BIN) -g -o ./c_src/main ./c_src/main.c -Ic_src -L ./target/debug -lrustc -lpthread -ldl
